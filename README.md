# 🦀 Programador CLI (Rust)

Um aplicativo de linha de comando (CLI) simples e interativo escrito em **Rust** para cadastrar e exibir informações de desenvolvedores, incluindo nome, idade e as linguagens de programação que dominam.

O projeto foi construído do zero com foco na prática dos conceitos fundamentais de gerenciamento de memória, *ownership*, empréstimos (*borrowing*), tratamento de dados e compilação cruzada (*cross-compilation*).

---

## 🚀 Funcionalidades

- 📥 **Captura Dinâmica:** Lê dados interativos diretamente do terminal (`std::io`).
- 🔄 **Tratamento de Strings e Tipos:** Limpeza de quebras de linha (`.trim()`) e conversão segura para números (`.parse()`).
- 📚 **Coleções Dinâmicas:** Uso de `Vec<String>` para armazenar uma quantidade indeterminada de linguagens de programação via `loop`.
- 🪟 **Suporte Multiplataforma:** Suporte para fechamento pausado no Windows (impede o fechamento instantâneo do Prompt de Comando).

---

## 🛠️ Conceitos de Rust Praticados

- **Ownership & Borrowing:** Uso de referências mutáveis (`&mut`) para modificação de memória via `read_line`.
- **Heap vs Stack:** Alocação dinâmica de memória com `String` e `Vec<String>`.
- **Structs:** Modelagem de dados com tipos compostos.
- **Pattern Matching:** Tratamento do encerramento de fluxo no terminal.
- **Cross-Compilation:** Compilação a partir do Linux para gerar binários nativos de Windows (`.exe`) via `mingw-w64`.

---

## 💻 Como Rodar o Projeto

### Pré-requisitos
Você precisará ter o [Rust e Cargo](https://www.rust-lang.org/tools/install) instalados em sua máquina.

### Executando Localmente

1. **Clone este repositório:**
   ```bash
   git clone [https://github.com/volpszz/cadastro-programadores](https://github.com/volpszz/cadastro-programadores)
   cd cadastro-programadores
