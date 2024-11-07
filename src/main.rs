use bitflags::bitflags;

bitflags! {
    #[derive(Default, Clone)]
    struct PermissaoFlags: u8 {
        const LEITURA = 0b100; // 4
        const ESCRITA = 0b010; // 2
        const EXECUCAO = 0b001; // 1
    }
}

#[derive(Clone)]
struct Arquivo { //Estrutura de Arquivo
    nome: String,
    tamanho: u64,
    permissoes: Permissao, 
    usuario: Usuario,
    grupo: Grupo,
}

#[derive(Clone)]
struct Permissao { //Estrutura de Permissão
    permissoes: PermissaoFlags,
    usuario: Usuario,
    grupo: String,
}

#[derive(Clone)]
struct Diretorio { //Estrutura de Diretório
    nome: String,
    arquivo: Vec<Arquivo>, 
    permissoes: Permissao,
    dono: Usuario,
}

#[derive(Clone)]
struct Usuario { //Estrutura de Usuário
    nome: String,
    uid: u16,
    grupo: Grupo,
}

#[derive(Clone)]
struct Grupo { //Estrutura de Grupo
    nome: String,
    gid: u16,
    membros: Vec<Usuario>,
}

//Implementação dos métodos das estruturas
//Implementação de Arquivo
impl Arquivo {
    //1º Método new
    fn new(nome: String, tamanho: u64, uid: u16, gid: u16) -> Arquivo {
        let usuario = Usuario{
            nome: format!("usuario{}", uid), //formando o nome do usuário UID da STRUCT
            uid,
            grupo: Grupo{
                nome: format!("grupo{}", gid), //formando o nome do grupo GID da STRUCT
                gid,
                membros: vec![], //usuário é relacionado ao grupo, por isso que tem um vetor de membros.
            },
        };  //Por fim, o arquivo é criado com o nome, tamanho, permissões, usuário e grupo.
        let grupo = usuario.grupo.clone(); //clone para não perder a referência
        //Graças ao [devire(Clone)] dentro do cargo.toml, podemos clonar a referência do usuário para o grupo.
        Arquivo {
            nome,
            tamanho,
            permissoes: Permissao::new(PermissaoFlags::LEITURA | PermissaoFlags::ESCRITA), 
            usuario, // uid
            grupo, // gid
        }
    }   

    //2º Método alterar_permissao
    fn alterar_permissao(&mut self, nova_permissao: Permissao) { 
        self.permissoes = nova_permissao;
    }

    //3º Método stat autorreferencial -> &self
    fn stat(&self) -> String {
        let arquivo = self.nome.clone();
        let tamanho = self.tamanho;
        let permissoes = self.permissoes.rwx();
        let usuario = self.usuario.nome.clone();
        let grupo = self.grupo.nome.clone();
        format!("{} {} {} {} {}", arquivo, tamanho, permissoes, usuario, grupo)
    }
}


impl Permissao {
    //1º Método new para criar uma nova permissão
    fn new(permissoes: PermissaoFlags) -> Permissao {
        Permissao {
            permissoes,
            usuario: Usuario {
                nome: "".to_string(),
                uid: 0,
                grupo: Grupo {
                    nome: "".to_string(),
                    gid: 0,
                    membros: vec![],
                },
            },
            grupo: String::new(),
        }
    }

    fn rwx(&self) -> String { // Método para retornar permissões em formato rwx
        format!(
            "{}{}{}",
            if self.permissoes.contains(PermissaoFlags::LEITURA) { "r" } else { "-" },
            if self.permissoes.contains(PermissaoFlags::ESCRITA) { "w" } else { "-" },
            if self.permissoes.contains(PermissaoFlags::EXECUCAO) { "x" } else { "-" },
        )
    }

    //2º Método octal autorreferente -> &self
    fn octal(&self) -> u8 {
        self.permissoes.bits()
    }

    // Método para verificar se tem permissão de leitura
    fn tem_leitura(&self) -> bool {
        self.permissoes.contains(PermissaoFlags::LEITURA)
    }

    // Método para verificar se tem permissão de escrita
    fn tem_escrita(&self) -> bool {
        self.permissoes.contains(PermissaoFlags::ESCRITA)
    }

    // Método para verificar se tem permissão de execução
    fn tem_execucao(&self) -> bool {
        self.permissoes.contains(PermissaoFlags::EXECUCAO)
    }
}   



impl Diretorio {
    //1º Método new
    fn new(nome: String, permissoes: Permissao, dono: Usuario, arquivo: Arquivo) -> Diretorio {
        Diretorio {
            nome,
            permissoes,
            dono,
            arquivo: vec![arquivo],
        }
    }

    //2º Método adicionar_arquivo
    fn adicionar_arquivo(&mut self, arquivo: Arquivo) {
        self.arquivo.push(arquivo);
    }

    //3º Método remover_arquivo
    fn remover_arquivo(&mut self, arquivo: Arquivo) {
        self.arquivo.retain(|a| a.nome != arquivo.nome);
    }

    //4º Método listar_arquivos
    fn listar_arquivos(&self) -> Vec<String> {
        self.arquivo.iter().map(|a| a.nome.clone()).collect()
    }
}

impl Usuario {
    //1º Método new
    fn new(nome: String, uid: u16, grupo: Grupo) -> Usuario {
        Usuario {
            nome,
            uid,
            grupo,
        }
    }

    //2º Método adiciona_grupo
    fn adiciona_grupo(&mut self, grupo: Grupo) {
        self.grupo = grupo;
    }

    //3º Método remove_grupo
    fn remove_grupo(&mut self) {
        self.grupo = Grupo::new("".to_string(), 0, vec![]);
    }

    //4º Método listar_grupos
    fn listar_grupos(&self) -> Vec<String> {
        self.grupo.membros.iter().map(|g| g.nome.clone()).collect()
    }
}

impl Grupo {
    //1º Método new
    fn new(nome: String, gid: u16, membros: Vec<Usuario>) -> Grupo {
        Grupo {
            nome,
            gid,
            membros,
        }
    }

    //2º Método adicionar_membro
    fn adicionar_membro(&mut self, usuario: Usuario) {
        self.membros.push(usuario);
    }

    //3º Método remover_membro
    fn remover_membro(&mut self, usuario: Usuario) {
        self.membros.retain(|u| u.nome != usuario.nome);
    }

    //4º Método listar_membros
    fn listar_membros(&self) -> Vec<String> {
        self.membros.iter().map(|u| u.nome.clone()).collect()
    }
}

// fn main(){
//     let arquivo = Arquivo::new("meuarquivo.txt".to_string(), 1024, 1, 1);
//     println!("{}", arquivo.stat());
// }


fn main() {
    // Criar usuários e grupos
    let grupo = Grupo::new("grupo1".to_string(), 1, vec![]);
    let usuario = Usuario::new("usuario1".to_string(), 1, grupo.clone());

    // Criar um arquivo
    let arquivo = Arquivo::new("meuarquivo.txt".to_string(), 1024, 1, 1);
    println!("Arquivo criado: {}", arquivo.stat());

    // Alterar permissões do arquivo
    let nova_permissao = Permissao::new(PermissaoFlags::LEITURA | PermissaoFlags::EXECUCAO);
    arquivo.alterar_permissao(nova_permissao);
    println!("Arquivo com nova permissão: {}", arquivo.stat());

    // Verificar permissões no formato rwx
    let permissoes = arquivo.permissoes.rwx();
    println!("Permissões do arquivo em formato rwx: {}", permissoes);

    // Verificar permissões no formato octal
    let permissoes_octal = arquivo.permissoes.octal();
    println!("Permissões do arquivo em formato octal: {}", permissoes_octal);

    // Criar um diretório
    let diretorio = Diretorio::new("meudiretorio".to_string(), Permissao::new(PermissaoFlags::LEITURA), usuario.clone(), arquivo);
    println!("Diretório criado com arquivos: {:?}", diretorio.listar_arquivos());

    // Adicionar e remover arquivos do diretório
    let novo_arquivo = Arquivo::new("outroarquivo.txt".to_string(), 2048, 2, 2);
    let mut diretorio_mut = diretorio;
    diretorio_mut.adicionar_arquivo(novo_arquivo);
    println!("Arquivos no diretório após adicionar: {:?}", diretorio_mut.listar_arquivos());

    diretorio_mut.remover_arquivo(Arquivo::new("outroarquivo.txt".to_string(), 2048, 2, 2));
    println!("Arquivos no diretório após remover: {:?}", diretorio_mut.listar_arquivos());

    // Testar adicionar e remover usuários de grupo
    let mut grupo_mut = grupo;
    grupo_mut.adicionar_membro(usuario.clone());
    println!("Membros do grupo após adicionar: {:?}", grupo_mut.listar_membros());

    grupo_mut.remover_membro(usuario);
    println!("Membros do grupo após remover: {:?}", grupo_mut.listar_membros());
}

