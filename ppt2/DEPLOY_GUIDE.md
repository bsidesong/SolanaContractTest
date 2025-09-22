# 🚀 Guia Completo de Deploy - TokenPo

Este guia fornece instruções passo a passo para fazer o deploy completo do sistema TokenPo na Solana Devnet.

## 📋 Pré-requisitos

Antes de começar, certifique-se de ter:

- [ ] Sistema operacional: Linux, macOS ou Windows (com WSL2)
- [ ] Conexão estável com a internet
- [ ] Pelo menos 2GB de espaço livre em disco
- [ ] Conhecimento básico de terminal/linha de comando

## 🛠️ Instalação do Ambiente

### Passo 1: Instalar Rust

```bash
# Baixar e instalar Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Escolha a opção 1 (instalação padrão)
# Após a instalação, recarregue o ambiente
source $HOME/.cargo/env

# Verificar instalação
rustc --version
cargo --version
```

**Saída esperada:**
```
rustc 1.70.0 (90c541806 2023-05-31)
cargo 1.70.0 (7fe40dc47 2023-04-27)
```

### Passo 2: Instalar Solana CLI

```bash
# Baixar e instalar Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/v1.18.4/install)"

# Adicionar ao PATH
export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"

# Adicionar permanentemente ao .bashrc ou .zshrc
echo 'export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"' >> ~/.bashrc

# Verificar instalação
solana --version
```

**Saída esperada:**
```
solana-cli 1.18.4 (src:devbuild; feat:3580551090, client:SolanaLabs)
```

### Passo 3: Instalar Anchor CLI

```bash
# Instalar Anchor CLI via Cargo
cargo install --git https://github.com/project-serum/anchor anchor-cli --locked

# Verificar instalação (pode demorar alguns minutos)
anchor --version
```

**Saída esperada:**
```
anchor-cli 0.29.0
```

### Passo 4: Instalar Node.js (para o frontend)

```bash
# Usando Node Version Manager (recomendado)
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash

# Recarregar terminal
source ~/.bashrc

# Instalar Node.js LTS
nvm install --lts
nvm use --lts

# Verificar instalação
node --version
npm --version
```

## ⚙️ Configuração do Ambiente Solana

### Passo 1: Configurar Rede

```bash
# Configurar para usar Devnet
solana config set --url https://api.devnet.solana.com

# Verificar configuração
solana config get
```

**Saída esperada:**
```
Config File: /home/user/.config/solana/cli/config.yml
RPC URL: https://api.devnet.solana.com
WebSocket URL: wss://api.devnet.solana.com/ (computed)
Keypair Path: /home/user/.config/solana/id.json
Commitment: confirmed
```

### Passo 2: Criar Carteira de Desenvolvimento

```bash
# Gerar nova carteira (se não tiver uma)
solana-keygen new --outfile ~/.config/solana/id.json

# Verificar chave pública
solana-keygen pubkey ~/.config/solana/id.json

# Verificar saldo (deve ser 0 inicialmente)
solana balance
```

### Passo 3: Obter SOL para Testes

```bash
# Solicitar airdrop de 2 SOL
solana airdrop 2

# Verificar saldo
solana balance
```

**Saída esperada:**
```
2 SOL
```

## 📁 Preparação do Projeto

### Passo 1: Criar Estrutura do Projeto

```bash
# Criar diretório do projeto
mkdir solana-ppt2
cd solana-ppt2

# Inicializar projeto Anchor
anchor init . --name ppt2
```

### Passo 2: Substituir Arquivos

Substitua os arquivos gerados pelos arquivos fornecidos:

1. **Substitua `src/lib.rs`** pelo conteúdo fornecido
2. **Substitua `Cargo.toml`** pelo conteúdo fornecido
3. **Substitua `Anchor.toml`** pelo conteúdo fornecido
4. **Adicione `game-updated.js`** na raiz do projeto
5. **Adicione `index-updated.html`** na raiz do projeto

### Passo 3: Instalar Dependências

```bash
# Instalar dependências do Anchor
anchor build
```

## 🔧 Configuração do Contrato

### Passo 1: Gerar Program ID

```bash
# Gerar chaves do programa
anchor keys list
```

**Saída esperada:**
```
ppt2: Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS
```

### Passo 2: Atualizar Program ID nos Arquivos

**⚠️ CRÍTICO**: Substitua o Program ID em todos os arquivos:

#### Em `Anchor.toml`:
```toml
[programs.devnet]
ppt2 = "SEU_PROGRAM_ID_AQUI"
```

#### Em `src/lib.rs`:
```rust
declare_id!("SEU_PROGRAM_ID_AQUI");
```

#### Em `game-updated.js`:
```javascript
const PROGRAM_ID = new PublicKey("SEU_PROGRAM_ID_AQUI");
```

### Passo 3: Criar Conta de Tesouraria

```bash
# Gerar chave para tesouraria
solana-keygen new --outfile treasury-keypair.json

# Obter chave pública da tesouraria
solana-keygen pubkey treasury-keypair.json
```

**Anote esta chave pública!** Você precisará dela no próximo passo.

### Passo 4: Atualizar Chave da Tesouraria

#### Em `game-updated.js`:
```javascript
const TREASURY_PUBKEY = new PublicKey("SUA_TREASURY_PUBKEY_AQUI");
```

## 🚀 Deploy do Contrato

### Passo 1: Compilar o Contrato

```bash
# Limpar builds anteriores
anchor clean

# Compilar o contrato
anchor build
```

**Saída esperada:**
```
BPF SDK: /home/user/.local/share/solana/install/active_release/bin/sdk/bpf
cargo-build-bpf child: rustup toolchain list -v
cargo-build-bpf child: cargo +bpf build --target bpfel-unknown-unknown --release
    Finished release [optimized] target(s) in 30.42s
cargo-build-bpf child: /home/user/.local/share/solana/install/active_release/bin/sdk/bpf/dependencies/bpf-tools/llvm/bin/llvm-readelf --dyn-symbols /home/user/solana-ppt2/target/deploy/ppt2.so

To deploy this program:
  $ solana program deploy /home/user/solana-ppt2/target/deploy/ppt2.so
The program address will default to this keypair (override with --program-id):
  /home/user/solana-ppt2/target/deploy/ppt2-keypair.json
```

### Passo 2: Deploy na Devnet

```bash
# Fazer deploy do programa
anchor deploy
```

**Saída esperada:**
```
Deploying workspace: https://api.devnet.solana.com
Upgrade authority: /home/user/.config/solana/id.json
Deploying program "ppt2"...
Program path: /home/user/solana-ppt2/target/deploy/ppt2.so...
Program Id: Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS

Deploy success
```

### Passo 3: Verificar Deploy

```bash
# Verificar se o programa foi deployado
solana program show SEU_PROGRAM_ID_AQUI
```

**Saída esperada:**
```
Program Id: Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS
Owner: BPFLoaderUpgradeab1e11111111111111111111111
ProgramData Address: ...
Authority: ...
Last Deployed In Slot: 123456789
Data Length: 123456 (0x1e240) bytes
Balance: 1.234 SOL
```

## 🌐 Configuração do Frontend

### Passo 1: Instalar Servidor Web

```bash
# Opção 1: Usar http-server (Node.js)
npm install -g http-server

# Opção 2: Usar Python (se disponível)
# python3 -m http.server 8000
```

### Passo 2: Servir os Arquivos

```bash
# Na pasta do projeto
http-server -p 8000 -c-1

# Ou com Python
# python3 -m http.server 8000
```

**Saída esperada:**
```
Starting up http-server, serving ./
Available on:
  http://127.0.0.1:8000
  http://192.168.1.100:8000
Hit CTRL-C to stop the server
```

### Passo 3: Instalar Phantom Wallet

1. Vá para https://phantom.app/
2. Clique em "Download"
3. Instale a extensão no seu navegador
4. Crie uma nova carteira ou importe uma existente
5. **IMPORTANTE**: Configure para usar Devnet:
   - Abra Phantom
   - Vá em Configurações → Developer Settings
   - Mude para "Devnet"

## 🎮 Teste do Sistema

### Passo 1: Acessar a Interface

1. Abra o navegador
2. Vá para `http://localhost:8000`
3. Abra `index-updated.html`

### Passo 2: Conectar Carteira

1. Clique em "Connect" no topo da página
2. Autorize a conexão com Phantom
3. Verifique se o endereço da carteira aparece

### Passo 3: Obter SOL de Teste

```bash
# No terminal, obter SOL para a carteira conectada
solana airdrop 1 ENDERECO_DA_SUA_CARTEIRA_PHANTOM
```

### Passo 4: Testar Pagamento

1. Na interface, clique em "Pagar 0.01 SOL (5 jogadas)"
2. Confirme a transação na Phantom
3. Aguarde a confirmação
4. Verifique se o contador de jogadas mudou para 5

### Passo 5: Testar Jogada

1. Clique em um dos botões (Rock, Paper, Scissors)
2. Aguarde o resultado
3. Verifique se o contador diminuiu para 4
4. Repita até acabarem as jogadas

## 🔍 Monitoramento e Debugging

### Verificar Transações

1. **Solana Explorer**: https://explorer.solana.com/?cluster=devnet
2. Cole o hash da transação para ver detalhes
3. Verifique logs do programa

### Logs em Tempo Real

```bash
# Monitorar logs do programa
solana logs SEU_PROGRAM_ID_AQUI
```

### Verificar Saldo da Tesouraria

```bash
# Verificar saldo da conta de tesouraria
solana balance SUA_TREASURY_PUBKEY_AQUI
```

### Debug do Frontend

1. Abra F12 no navegador
2. Vá para a aba "Console"
3. Procure por erros em vermelho
4. Verifique a aba "Network" para problemas de conexão

## ❌ Solução de Problemas Comuns

### Erro: "Program not found"

**Causa**: Program ID incorreto ou deploy falhou
**Solução**:
```bash
# Verificar se o programa existe
solana program show SEU_PROGRAM_ID

# Se não existir, fazer deploy novamente
anchor deploy
```

### Erro: "Insufficient funds"

**Causa**: Não há SOL suficiente na carteira
**Solução**:
```bash
# Obter mais SOL
solana airdrop 2
```

### Erro: "Transaction simulation failed"

**Causa**: Erro na lógica do contrato ou parâmetros incorretos
**Solução**:
1. Verificar logs do programa: `solana logs SEU_PROGRAM_ID`
2. Verificar se todos os parâmetros estão corretos
3. Verificar se a conta foi inicializada

### Frontend não carrega

**Causa**: Servidor não está rodando ou arquivos não encontrados
**Solução**:
1. Verificar se o servidor está rodando: `http://localhost:8000`
2. Verificar se os arquivos estão no diretório correto
3. Verificar console do navegador para erros

### Phantom não conecta

**Causa**: Phantom não está configurado para Devnet
**Solução**:
1. Abrir Phantom
2. Ir em Settings → Developer Settings
3. Mudar para "Devnet"
4. Recarregar a página

## 📊 Verificação Final

### Checklist de Deploy Bem-sucedido

- [ ] Rust instalado e funcionando
- [ ] Solana CLI instalado e configurado para Devnet
- [ ] Anchor CLI instalado
- [ ] Carteira criada e com SOL de teste
- [ ] Projeto compilado sem erros
- [ ] Program ID atualizado em todos os arquivos
- [ ] Conta de tesouraria criada
- [ ] Contrato deployado com sucesso
- [ ] Frontend servindo corretamente
- [ ] Phantom configurado para Devnet
- [ ] Conexão com carteira funcionando
- [ ] Pagamento de 0.01 SOL funcionando
- [ ] Jogadas sendo contabilizadas corretamente
- [ ] Fundos chegando na tesouraria

### Comandos de Verificação

```bash
# Verificar programa deployado
solana program show SEU_PROGRAM_ID

# Verificar saldo da tesouraria
solana balance SUA_TREASURY_PUBKEY

# Verificar logs recentes
solana logs SEU_PROGRAM_ID --limit 10
```

## 🎉 Parabéns!

Se chegou até aqui, seu sistema TokenPo está funcionando! 

### Próximos Passos

1. **Teste Extensivamente**: Faça várias jogadas para garantir estabilidade
2. **Monitore a Tesouraria**: Verifique se os fundos estão sendo coletados
3. **Personalize**: Modifique cores, textos e funcionalidades
4. **Documente**: Anote qualquer modificação que fizer

### Para Produção (Mainnet)

⚠️ **ATENÇÃO**: Antes de ir para mainnet:
1. Faça auditoria completa do código
2. Teste por semanas na devnet
3. Considere contratar auditoria profissional
4. Configure monitoramento robusto
5. Tenha plano de contingência

---

**Boa sorte com seu projeto! 🚀🎮**

