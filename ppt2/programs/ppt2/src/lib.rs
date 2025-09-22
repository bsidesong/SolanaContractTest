
use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;
use anchor_lang::solana_program::system_program;
use anchor_lang::solana_program::clock::Clock;
use anchor_lang::solana_program::hash::hash;


declare_id!("5ccZFdZ3eQxiN6vvYcAurVcSrMPsQHcwNZFZSfFDzv8J"); // Substitua pelo seu Program ID gerado

#[program]
pub mod ppt2 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let game_state = &mut ctx.accounts.game_state;
        game_state.player = *ctx.accounts.player.key;
        game_state.plays_left = 0;
        game_state.last_paid_slot = 0;
        game_state.score = 0;
        game_state.history = Vec::new();
        Ok(())
    }

    pub fn make_play(ctx: Context<MakePlay>, player_move: u8) -> Result<()> {
        let game_state = &mut ctx.accounts.game_state;
        let _player = &ctx.accounts.player; // Usado para validação, mas não diretamente aqui
        let _treasury = &ctx.accounts.treasury; // Usado para validação, mas não diretamente aqui

        // Verificar se o jogador tem jogadas restantes
        if game_state.plays_left == 0 {
            return err!(ErrorCode::NoPlaysLeft);
        }

        // Decrementar jogadas restantes
        game_state.plays_left -= 1;

        // Gerar uma escolha aleatória para o programa (0: pedra, 1: papel, 2: tesoura)
        let clock = Clock::get()?;
        let program_move = (hash(&clock.slot.to_le_bytes()).to_bytes()[0] % 3) as u8;

        // Determinar o resultado da partida
        let result = match (player_move, program_move) {
            (0, 0) | (1, 1) | (2, 2) => 1, // Empate
            (0, 2) | (1, 0) | (2, 1) => 2, // Vitória do jogador
            _ => 0, // Derrota do jogador
        };

        // Atualizar o score do jogador
        game_state.score += result as u64;

        // Adicionar o resultado da partida ao histórico
        let match_result = MatchResult {
            player_choice: player_move,
            program_choice: program_move as u8,
            result,
        };

        game_state.history.push(match_result);

        // Limitar o histórico a 500 partidas
        if game_state.history.len() > 500 {
            game_state.history.remove(0);
        }

        Ok(())
    }

    pub fn pay_for_plays(ctx: Context<PayForPlays>) -> Result<()> {
        let game_state = &mut ctx.accounts.game_state;
        let player = &ctx.accounts.player;
        let treasury = &ctx.accounts.treasury;

        // Transferir 0.01 SOL (10_000_000 lamports) do jogador para a conta de tesouraria
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &player.key(),
            &treasury.key(),
            10_000_000, // 0.01 SOL
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                player.to_account_info(),
                treasury.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        // Conceder 5 jogadas ao jogador
        game_state.plays_left += 5;
        game_state.last_paid_slot = Clock::get()?.slot;

        Ok(())
    }

    pub fn withdraw_treasury(ctx: Context<WithdrawTreasury>, amount: u64) -> Result<()> {
        let treasury = &mut ctx.accounts.treasury;
        let admin = &ctx.accounts.admin;

        // Apenas o administrador pode sacar. O admin deve ser uma chave específica, não o program_id.
        // Para este exemplo, vamos assumir que o `admin` é o `payer` que fez o deploy do programa.
        // Em um cenário real, você teria uma conta de administrador configurada no programa.
        // Por simplicidade, vamos permitir que o `admin` seja o `payer` da transação.
        // Se você quiser um admin fixo, você pode armazenar a pubkey do admin no GameState ou em outra conta.
        // Por enquanto, a validação de que `admin` é o `Signer` já é feita pelo Anchor.

        // Transferir fundos da tesouraria para o administrador
        **treasury.to_account_info().try_borrow_mut_lamports()? -= amount;
        **admin.to_account_info().try_borrow_mut_lamports()? += amount;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = player, space = 8 + GameState::LEN)]
    pub game_state: Account<'info, GameState>,
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MakePlay<'info> {
    #[account(mut, has_one = player)]
    pub game_state: Account<'info, GameState>,
    #[account(mut)]
    pub player: Signer<'info>,
    /// CHECK: A conta de tesouraria é apenas para receber fundos
    #[account(mut)]
    pub treasury: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct PayForPlays<'info> {
    #[account(mut, has_one = player)]
    pub game_state: Account<'info, GameState>,
    #[account(mut)]
    pub player: Signer<'info>,
    /// CHECK: A conta de tesouraria é apenas para receber fundos
    #[account(mut)]
    pub treasury: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WithdrawTreasury<'info> {
    /// CHECK: A conta de tesouraria é apenas para receber fundos
    #[account(mut)]
    pub treasury: AccountInfo<'info>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct GameState {
    pub player: Pubkey,
    pub plays_left: u8,
    pub last_paid_slot: u64,
    pub score: u64,
    pub history: Vec<MatchResult>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub struct MatchResult {
    pub player_choice: u8,
    pub program_choice: u8,
    pub result: u8, // 0: derrota, 1: empate, 2: vitória
}

impl GameState {
    // Tamanho máximo para 500 partidas no histórico
    // Pubkey (32) + u8 (1) + u64 (8) + u64 (8) + Vec<MatchResult> (4 bytes para o tamanho do vetor + 500 * (u8 + u8 + u8))
    const LEN: usize = 32 + 1 + 8 + 8 + (4 + 500 * (1 + 1 + 1));
}

#[error_code]
pub enum ErrorCode {
    #[msg("No plays left. Please pay to get more plays.")]
    NoPlaysLeft,
    #[msg("Unauthorized to perform this action.")]
    Unauthorized,
}


