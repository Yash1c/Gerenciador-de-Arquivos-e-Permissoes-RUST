use std::io;
use std::io::Write;
use bitflags::bitflags;

bitflags! {
    #[derive(Default, Clone)]
    struct PermissaoFlags: u8 {
        const LEITURA = 0b100;
        const ESCRITA = 0b010;
        const EXECUCAO = 0b001;
    }
}

#[derive(Clone)]
struct Arquivo {
    nome: String,
    tamanho: u64,
    permissoes: PermissaoFlags,
    usuario: Usuario,
    grupo: Grupo,
}

#[derive(Clone)]
struct Usuario {
    nome: String,
    uid: u16,
}

#[derive(Clone)]
struct Grupo {
    nome: String,
    gid: u16,
    membros: Vec<Usuario>,
}

impl Arquivo {
    fn new(nome: String, tamanho: u64, usuario: Usuario, grupo: Grupo) -> Arquivo {
        Arquivo {
            nome,
            tamanho,
            permissoes: PermissaoFlags::LEITURA | PermissaoFlags::ESCRITA,
            usuario,
            grupo,
        }
    }

    fn alterar_permissao(&mut self, leitura: bool, escrita: bool, execucao: bool) {
        let mut novas_permissoes = PermissaoFlags::empty();
        if leitura {
            novas_permissoes |= PermissaoFlags::LEITURA;
        }
        if escrita {
            novas_permissoes |= PermissaoFlags::ESCRITA;
        }
        if execucao {
            novas_permissoes |= PermissaoFlags::EXECUCAO;
        }
        self.permissoes = novas_permissoes;
    }

    fn stat(&self) -> String {
        format!(
            "{} {} {} {} {}",
            self.nome,
            self.tamanho,
            self.permissoes_rwx(),
            self.usuario.nome,
            self.grupo.nome
        )
    }

    fn permissoes_rwx(&self) -> String {
        format!(
            "{}{}{}",
            if self.permissoes.contains(PermissaoFlags::LEITURA) { "r" } else { "-" },
            if self.permissoes.contains(PermissaoFlags::ESCRITA) { "w" } else { "-" },
            if self.permissoes.contains(PermissaoFlags::EXECUCAO) { "x" } else { "-" },
        )
    }
}

impl Usuario {
    fn new(nome: String, uid: u16) -> Usuario {
        Usuario { nome, uid }
    }
}

impl Grupo {
    fn new(nome: String, gid: u16) -> Grupo {
        Grupo {
            nome,
            gid,
            membros: Vec::new(),
        }
    }

//Comecei a tentar criar, com uso da biblioteca hashlib, para usuarios e grupos.
    fn cadastrar_usuario(&mut self, usuario: Usuario) {
        self.membros.push(usuario);
    }
}

fn main() {
    let mut arquivos = Vec::new();
    let mut usuarios: Vec<Usuario> = Vec::new();
    let mut grupos: Vec<Grupo> = Vec::new();

    loop {
        println!("\n--- Menu de Gerenciamento do Sistema de Arquivos ---");

        // Arquivo
        println!("1. Criar arquivo");
        println!("2. Editar permissões do arquivo");
        println!("3. Listar arquivos");
        println!("4. Apagar arquivo");

        // Usuário
        println!("5. Criar usuário");
        println!("6. Listar usuários");

        // Grupo
        println!("7. Criar grupo");
        println!("8. Listar grupos");

        println!("0. Sair");

        print!("Escolha uma opção: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Falha ao ler a entrada");

        match input.trim() {
            "1" => {
                
                let mut nome = String::new();
                print!("Nome do arquivo: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut nome).expect("Falha ao ler a entrada");
                let nome = nome.trim().to_string();

                let mut tamanho = String::new();
                print!("Tamanho do arquivo: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut tamanho).expect("Falha ao ler a entrada");
                let tamanho = tamanho.trim().parse::<u64>().unwrap();

                if usuarios.is_empty() || grupos.is_empty() {
                    println!("Crie um usuário e um grupo antes de criar um arquivo.");
                    continue;
                }
                
                let usuario = usuarios[0].clone(); 
                let grupo = grupos[0].clone(); 
                
                let arquivo = Arquivo::new(nome, tamanho, usuario, grupo);
                println!("Arquivo criado com sucesso: {}", arquivo.stat());

                arquivos.push(arquivo);
            }
            "2" => {
                // Editar permissões do arquivo
                if arquivos.is_empty() {
                    println!("Nenhum arquivo para editar.");
                    continue;
                }

                println!("Arquivos disponíveis:");
                for (i, arquivo) in arquivos.iter().enumerate() {
                    println!("{} - {}", i + 1, arquivo.stat());
                }

                let mut indice = String::new();
                print!("Digite o número do arquivo que deseja editar: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut indice).expect("Falha ao ler a entrada");
                let indice = indice.trim().parse::<usize>().unwrap() - 1;

                if indice >= arquivos.len() {
                    println!("Índice inválido.");
                    continue;
                }

                let mut leitura = String::new();
                print!("Permissão de leitura (s/n): ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut leitura).expect("Falha ao ler a entrada");
                let leitura = leitura.trim() == "s";

                let mut escrita = String::new();
                print!("Permissão de escrita (s/n): ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut escrita).expect("Falha ao ler a entrada");
                let escrita = escrita.trim() == "s";

                let mut execucao = String::new();
                print!("Permissão de execução (s/n): ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut execucao).expect("Falha ao ler a entrada");
                let execucao = execucao.trim() == "s";

                arquivos[indice].alterar_permissao(leitura, escrita, execucao);
                println!("Permissões alteradas com sucesso!");
            }
            "3" => {
                // Listar arquivos
                if arquivos.is_empty() {
                    println!("Nenhum arquivo encontrado.");
                } else {
                    for arquivo in &arquivos {
                        println!("Arquivo: {}", arquivo.stat());
                    }
                }
            }
            "4" => {
                // Apagar arquivo
                if arquivos.is_empty() {
                    println!("Nenhum arquivo para apagar.");
                    continue;
                }

                let mut indice = String::new();
                print!("Digite o número do arquivo que deseja apagar: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut indice).expect("Falha ao ler a entrada");
                let indice = indice.trim().parse::<usize>().unwrap() - 1;

                if indice < arquivos.len() {
                    arquivos.remove(indice);
                    println!("Arquivo apagado com sucesso.");
                } else {
                    println!("Índice inválido.");
                }
            }
            "5" => {
                // Criar usuário
                let mut nome = String::new();
                print!("Nome do usuário: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut nome).expect("Falha ao ler a entrada");
                let usuario = Usuario::new(nome.trim().to_string(), usuarios.len() as u16);
                usuarios.push(usuario);
                println!("Usuário criado com sucesso.");
            }
            "6" => {
                // Listar usuários
                if usuarios.is_empty() {
                    println!("Nenhum usuário encontrado.");
                } else {
                    for usuario in &usuarios {
                        println!("Usuários: {} (UID: {})", usuario.nome, usuario.uid);
                    }
                }
            }
            "7" => {
                // Criar grupo
                let mut nome = String::new();
                print!("Nome do grupo: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut nome).expect("Falha ao ler a entrada");
                let grupo = Grupo::new(nome.trim().to_string(), grupos.len() as u16);
                grupos.push(grupo);
                println!("Grupo criado com sucesso.");
            }
            "8" => {
                // Listar grupos
                if grupos.is_empty() {
                    println!("Nenhum grupo encontrado.");
                } else {
                    for grupo in &grupos {
                        println!("Grupo: {} (GID: {})", grupo.nome, grupo.gid);
                    }
                }
            }
            "0" => {
                println!("Saindo do programa...");
                break;
            }
            _ => println!("Opção inválida, tente novamente."),
        }
    }
}
