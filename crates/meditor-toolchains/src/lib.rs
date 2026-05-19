use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::Path;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToolchainDomain {
    Assembly,
    C,
    Cpp,
    ObjectiveC,
    Ada,
    Fortran,
    Cobol,
    Basic,
    Clipper,
    Rust,
    Java,
    JavaScript,
    Php,
    Python,
    Perl,
    Julia,
    R,
    Prolog,
    Lisp,
    DotNet,
    Go,
    Ruby,
    Lua,
    Functional,
    Swift,
    Database,
    Markup,
    Infrastructure,
    VersionControl,
    Setup,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LanguageSupportKind {
    ProgrammingLanguage,
    LanguageFamily,
    Runtime,
    Framework,
    Markup,
    DatabaseDialect,
    Infrastructure,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LanguageSupportState {
    Active,
    MissingTooling,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OpenSourceTooling {
    pub id: &'static str,
    pub display_name: &'static str,
    pub executable_ids: Vec<&'static str>,
    pub source_url: &'static str,
    pub documentation_url: &'static str,
    pub install_strategy: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProgrammingLanguageSupport {
    pub id: &'static str,
    pub display_name: &'static str,
    pub kind: LanguageSupportKind,
    pub domain: ToolchainDomain,
    pub file_extensions: Vec<&'static str>,
    pub activation_all_of: Vec<&'static str>,
    pub activation_any_of: Vec<&'static str>,
    pub tooling: Vec<OpenSourceTooling>,
    pub install_action: &'static str,
    pub configure_action: &'static str,
    pub network_test_url: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LanguageSupportStatus {
    pub language_id: &'static str,
    pub display_name: &'static str,
    pub state: LanguageSupportState,
    pub detected_required_executables: Vec<&'static str>,
    pub missing_required_executables: Vec<&'static str>,
    pub install_action: &'static str,
    pub configure_action: &'static str,
}

impl LanguageSupportStatus {
    pub fn is_active(&self) -> bool {
        self.state == LanguageSupportState::Active
    }
}

impl ProgrammingLanguageSupport {
    pub fn status_with<F>(&self, is_detected: F) -> LanguageSupportStatus
    where
        F: Fn(&str) -> bool,
    {
        let mut detected_required_executables = Vec::new();
        let mut missing_required_executables = Vec::new();

        for executable in &self.activation_all_of {
            if is_detected(executable) {
                detected_required_executables.push(*executable);
            } else {
                missing_required_executables.push(*executable);
            }
        }

        if !self.activation_any_of.is_empty() {
            let any_detected = self
                .activation_any_of
                .iter()
                .any(|executable| is_detected(executable));
            if any_detected {
                detected_required_executables.extend(
                    self.activation_any_of
                        .iter()
                        .copied()
                        .filter(|executable| is_detected(executable)),
                );
            } else {
                missing_required_executables.extend(self.activation_any_of.iter().copied());
            }
        }

        let state = if missing_required_executables.is_empty() {
            LanguageSupportState::Active
        } else {
            LanguageSupportState::MissingTooling
        };

        LanguageSupportStatus {
            language_id: self.id,
            display_name: self.display_name,
            state,
            detected_required_executables,
            missing_required_executables,
            install_action: self.install_action,
            configure_action: self.configure_action,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExecutableRequirement {
    pub id: &'static str,
    pub display_name: &'static str,
    pub domain: ToolchainDomain,
    pub required_for_core: bool,
    pub configure_action: &'static str,
}

pub fn seed_executable_requirements() -> Vec<ExecutableRequirement> {
    vec![
        exe("cargo", "Cargo", ToolchainDomain::Rust, true),
        exe("rustc", "Rust compiler", ToolchainDomain::Rust, true),
        exe(
            "rust-analyzer",
            "Rust Analyzer",
            ToolchainDomain::Rust,
            false,
        ),
        exe("java", "Java runtime", ToolchainDomain::Java, true),
        exe("javac", "Java compiler", ToolchainDomain::Java, false),
        exe("mvn", "Maven", ToolchainDomain::Java, false),
        exe("gradle", "Gradle", ToolchainDomain::Java, false),
        exe(
            "native-image",
            "GraalVM Native Image",
            ToolchainDomain::Java,
            false,
        ),
        exe("node", "Node.js", ToolchainDomain::JavaScript, true),
        exe("npm", "npm", ToolchainDomain::JavaScript, true),
        exe("pnpm", "pnpm", ToolchainDomain::JavaScript, false),
        exe("yarn", "Yarn", ToolchainDomain::JavaScript, false),
        exe("deno", "Deno", ToolchainDomain::JavaScript, false),
        exe("bun", "Bun", ToolchainDomain::JavaScript, false),
        exe("php", "PHP", ToolchainDomain::Php, false),
        exe("composer", "Composer", ToolchainDomain::Php, false),
        exe("python", "Python", ToolchainDomain::Python, true),
        exe("python3", "Python 3", ToolchainDomain::Python, true),
        exe("perl", "Perl", ToolchainDomain::Perl, false),
        exe("gcc", "GCC", ToolchainDomain::C, false),
        exe("clang", "Clang", ToolchainDomain::Cpp, false),
        exe("cmake", "CMake", ToolchainDomain::Cpp, false),
        exe("make", "Make", ToolchainDomain::Cpp, false),
        exe("nasm", "NASM", ToolchainDomain::Assembly, false),
        exe("as", "GNU assembler", ToolchainDomain::Assembly, false),
        exe("gnat", "GNAT Ada compiler", ToolchainDomain::Ada, false),
        exe("gprbuild", "GPRbuild", ToolchainDomain::Ada, false),
        exe("gfortran", "GNU Fortran", ToolchainDomain::Fortran, false),
        exe("cobc", "GnuCOBOL", ToolchainDomain::Cobol, false),
        exe("fbc", "FreeBASIC", ToolchainDomain::Basic, false),
        exe(
            "qb64pe",
            "QB64 Phoenix Edition",
            ToolchainDomain::Basic,
            false,
        ),
        exe("harbour", "Harbour", ToolchainDomain::Clipper, false),
        exe("julia", "Julia", ToolchainDomain::Julia, false),
        exe("R", "R", ToolchainDomain::R, false),
        exe("swipl", "SWI-Prolog", ToolchainDomain::Prolog, false),
        exe("gprolog", "GNU Prolog", ToolchainDomain::Prolog, false),
        exe("sbcl", "SBCL", ToolchainDomain::Lisp, false),
        exe("clisp", "CLISP", ToolchainDomain::Lisp, false),
        exe("racket", "Racket", ToolchainDomain::Lisp, false),
        exe("guile", "Guile", ToolchainDomain::Lisp, false),
        exe("sqlite3", "SQLite", ToolchainDomain::Setup, true),
        exe(
            "psql",
            "PostgreSQL client",
            ToolchainDomain::Database,
            false,
        ),
        exe("mysql", "MySQL client", ToolchainDomain::Database, false),
        exe(
            "sqlcmd",
            "SQL Server sqlcmd",
            ToolchainDomain::Database,
            false,
        ),
        exe("sqlplus", "SQL*Plus", ToolchainDomain::Database, false),
        exe("git", "Git", ToolchainDomain::VersionControl, true),
        exe("svn", "SVN", ToolchainDomain::VersionControl, false),
        exe("go", "Go", ToolchainDomain::Go, false),
        exe("dotnet", ".NET SDK", ToolchainDomain::DotNet, false),
        exe("ruby", "Ruby", ToolchainDomain::Ruby, false),
        exe("lua", "Lua", ToolchainDomain::Lua, false),
        exe("ghc", "GHC Haskell", ToolchainDomain::Functional, false),
        exe("erl", "Erlang", ToolchainDomain::Functional, false),
        exe("elixir", "Elixir", ToolchainDomain::Functional, false),
        exe("dart", "Dart", ToolchainDomain::JavaScript, false),
        exe("swift", "Swift", ToolchainDomain::Swift, false),
        exe(
            "terraform",
            "Terraform",
            ToolchainDomain::Infrastructure,
            false,
        ),
        exe("kubectl", "kubectl", ToolchainDomain::Infrastructure, false),
        exe("helm", "Helm", ToolchainDomain::Infrastructure, false),
        exe("ansible", "Ansible", ToolchainDomain::Infrastructure, false),
        exe("nix", "Nix", ToolchainDomain::Infrastructure, false),
        exe("pwsh", "PowerShell", ToolchainDomain::Infrastructure, false),
    ]
}

fn exe(
    id: &'static str,
    display_name: &'static str,
    domain: ToolchainDomain,
    required_for_core: bool,
) -> ExecutableRequirement {
    ExecutableRequirement {
        id,
        display_name,
        domain,
        required_for_core,
        configure_action: "Settings > Languages & Toolchains > Configure Executable",
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ToolchainProfile {
    pub name: String,
    pub executable_id: String,
    pub path: Option<String>,
}

impl ToolchainProfile {
    pub fn missing(executable_id: impl Into<String>) -> Self {
        Self {
            name: "missing".to_string(),
            executable_id: executable_id.into(),
            path: None,
        }
    }

    pub fn configured(executable_id: impl Into<String>, path: impl Into<String>) -> Self {
        Self {
            name: "configured".to_string(),
            executable_id: executable_id.into(),
            path: Some(path.into()),
        }
    }

    pub fn requires_user_configuration(&self) -> bool {
        self.path.is_none()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ToolingNetworkDialog {
    pub id: &'static str,
    pub title: &'static str,
    pub fields: Vec<&'static str>,
    pub direct_test_action: &'static str,
    pub proxy_test_action: &'static str,
    pub validation_success_action: &'static str,
    pub test_targets: Vec<NetworkTestTarget>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NetworkTestTarget {
    pub id: &'static str,
    pub label: &'static str,
    pub url: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProxyValidationState {
    Empty,
    InvalidUrl,
    Reachable,
    Unreachable,
}

pub fn supported_language_catalog() -> Vec<ProgrammingLanguageSupport> {
    vec![
        lang("assembly", "Assembly", LanguageSupportKind::LanguageFamily, ToolchainDomain::Assembly, &["asm", "s", "S"], &[], &["nasm", "as", "clang"], vec![
            tool("nasm", "NASM", &["nasm"], "https://www.nasm.us/", "https://www.nasm.us/docs.php", "Download NASM or install through the platform package manager."),
            tool("binutils", "GNU assembler", &["as"], "https://www.gnu.org/software/binutils/", "https://sourceware.org/binutils/docs/as/", "Install GNU binutils through the platform package manager."),
            tool("llvm", "LLVM integrated assembler", &["clang"], "https://llvm.org/", "https://clang.llvm.org/docs/CommandGuide/clang.html", "Install LLVM/Clang through the platform package manager."),
        ]),
        lang("c", "C", LanguageSupportKind::ProgrammingLanguage, ToolchainDomain::C, &["c", "h"], &[], &["gcc", "clang"], c_family_tooling()),
        lang("cpp", "C++", LanguageSupportKind::ProgrammingLanguage, ToolchainDomain::Cpp, &["cc", "cpp", "cxx", "hpp", "hh"], &[], &["g++", "clang++", "gcc", "clang"], c_family_tooling()),
        lang("objective-c", "Objective-C", LanguageSupportKind::ProgrammingLanguage, ToolchainDomain::ObjectiveC, &["m", "h"], &[], &["clang", "gcc"], c_family_tooling()),
        lang("objective-cpp", "Objective-C++", LanguageSupportKind::ProgrammingLanguage, ToolchainDomain::ObjectiveC, &["mm", "M"], &[], &["clang++", "g++", "clang", "gcc"], c_family_tooling()),
        lang("ada", "Ada", LanguageSupportKind::ProgrammingLanguage, ToolchainDomain::Ada, &["adb", "ads", "gpr"], &[], &["gnat", "gprbuild"], vec![
            tool("gnat", "GNAT Ada in GCC", &["gnat"], "https://gcc.gnu.org/onlinedocs/gnat_ugn/", "https://gcc.gnu.org/onlinedocs/gnat_ugn/", "Install GNAT/GCC Ada through the platform package manager."),
            tool("ada-language-server", "Ada Language Server", &["ada_language_server"], "https://github.com/AdaCore/ada_language_server", "https://github.com/AdaCore/ada_language_server", "Download the Ada Language Server release or install through Alire where available."),
        ]),
        lang("fortran", "Fortran", LanguageSupportKind::ProgrammingLanguage, ToolchainDomain::Fortran, &["f", "f90", "f95", "f03", "f08"], &[], &["gfortran", "flang"], vec![
            tool("gfortran", "GNU Fortran", &["gfortran"], "https://gcc.gnu.org/fortran/", "https://gcc.gnu.org/onlinedocs/gfortran/", "Install GCC Fortran through the platform package manager."),
            tool("flang", "LLVM Flang", &["flang"], "https://flang.llvm.org/", "https://flang.llvm.org/docs/", "Install LLVM Flang where available for the platform."),
        ]),
        lang("cobol", "COBOL", LanguageSupportKind::ProgrammingLanguage, ToolchainDomain::Cobol, &["cob", "cbl", "cpy"], &[], &["cobc"], vec![
            tool("gnucobol", "GnuCOBOL", &["cobc"], "https://gnucobol.sourceforge.io/", "https://gnucobol.sourceforge.io/guides.html", "Install GnuCOBOL through the platform package manager or from source."),
            tool("gcc-cobol", "GCC COBOL front end", &["gcobol"], "https://gcc.gnu.org/", "https://gcc.gnu.org/onlinedocs/", "Use when the GCC COBOL front end is available for the target platform."),
        ]),
        lang("basic", "BASIC", LanguageSupportKind::LanguageFamily, ToolchainDomain::Basic, &["bas", "bi"], &[], &["fbc", "qb64pe"], vec![
            tool("freebasic", "FreeBASIC", &["fbc"], "https://www.freebasic.net/", "https://www.freebasic.net/wiki/DocToc", "Download FreeBASIC or install through the platform package manager where available."),
            tool("qb64pe", "QB64 Phoenix Edition", &["qb64pe"], "https://github.com/QB64-Phoenix-Edition/QB64pe", "https://qb64phoenix.com/qb64wiki/", "Download QB64 Phoenix Edition for the platform."),
        ]),
        lang("clipper-xbase", "Clipper/xBase", LanguageSupportKind::LanguageFamily, ToolchainDomain::Clipper, &["prg", "ch", "hb"], &[], &["harbour", "xharbour"], vec![
            tool("harbour", "Harbour", &["harbour"], "https://harbour.github.io/", "https://harbour.github.io/doc/", "Install Harbour from source, package manager, or project binaries where available."),
            tool("xharbour", "xHarbour", &["xharbour"], "https://www.xharbour.org/", "https://www.xharbour.org/index.asp?page=product/docs", "Install xHarbour where available for the platform."),
        ]),
        lang("rust", "Rust", LanguageSupportKind::ProgrammingLanguage, ToolchainDomain::Rust, &["rs"], &["rustc", "cargo"], &[], vec![
            tool("rustup", "rustup, rustc, cargo", &["rustc", "cargo"], "https://www.rust-lang.org/tools/install", "https://doc.rust-lang.org/book/", "Install with rustup and add rustfmt, clippy, and rust-analyzer components."),
            tool("rust-analyzer", "rust-analyzer", &["rust-analyzer"], "https://rust-analyzer.github.io/", "https://rust-analyzer.github.io/manual.html", "Install via rustup component or release package."),
        ]),
        lang("python", "Python", LanguageSupportKind::ProgrammingLanguage, ToolchainDomain::Python, &["py", "pyw"], &[], &["python", "python3"], vec![
            tool("cpython", "CPython", &["python", "python3"], "https://www.python.org/downloads/", "https://docs.python.org/3/", "Download CPython or install through the platform package manager."),
            tool("pypy", "PyPy", &["pypy3"], "https://www.pypy.org/", "https://doc.pypy.org/", "Install PyPy as an optional interpreter profile."),
            tool("ruff", "Ruff", &["ruff"], "https://github.com/astral-sh/ruff", "https://docs.astral.sh/ruff/", "Install with pipx, uv, or the platform package manager."),
        ]),
        lang("php", "PHP", LanguageSupportKind::ProgrammingLanguage, ToolchainDomain::Php, &["php"], &["php"], &[], vec![
            tool("php-src", "PHP source and Zend Engine", &["php"], "https://github.com/php/php-src", "https://www.php.net/docs.php", "Install PHP and Composer through the platform package manager."),
            tool("composer", "Composer", &["composer"], "https://getcomposer.org/", "https://getcomposer.org/doc/", "Download Composer and register it as the PHP package manager."),
            tool("peachpie", "PeachPie", &["dotnet"], "https://www.peachpie.io/", "https://docs.peachpie.io/", "Install .NET SDK and PeachPie templates for compatible PHP compilation."),
            tool("kphp", "KPHP", &["kphp"], "https://vkcom.github.io/kphp/", "https://vkcom.github.io/kphp/", "Install KPHP for compatible PHP-to-C++ compilation workflows."),
        ]),
        lang("perl", "Perl", LanguageSupportKind::ProgrammingLanguage, ToolchainDomain::Perl, &["pl", "pm", "t"], &["perl"], &[], vec![
            tool("perl", "Perl", &["perl"], "https://www.perl.org/get.html", "https://perldoc.perl.org/", "Install Perl through the platform package manager or from perl.org."),
            tool("perl-language-server", "Perl Language Server", &["perl"], "https://github.com/richterger/Perl-LanguageServer", "https://github.com/richterger/Perl-LanguageServer", "Install Perl::LanguageServer through CPAN."),
        ]),
        lang("java", "Java/JVM", LanguageSupportKind::LanguageFamily, ToolchainDomain::Java, &["java", "jav"], &["java"], &[], java_tooling()),
        lang("kotlin", "Kotlin", LanguageSupportKind::ProgrammingLanguage, ToolchainDomain::Java, &["kt", "kts"], &[], &["kotlinc", "gradle", "mvn"], java_tooling()),
        lang("scala", "Scala", LanguageSupportKind::ProgrammingLanguage, ToolchainDomain::Java, &["scala", "sc"], &[], &["scala", "scalac", "sbt"], java_tooling()),
        lang("groovy", "Groovy", LanguageSupportKind::ProgrammingLanguage, ToolchainDomain::Java, &["groovy", "gradle"], &[], &["groovy", "gradle"], java_tooling()),
        lang("clojure", "Clojure", LanguageSupportKind::ProgrammingLanguage, ToolchainDomain::Java, &["clj", "cljs", "cljc", "edn"], &[], &["clojure", "clj"], java_tooling()),
        lang("graalvm-native-image", "GraalVM Native Image", LanguageSupportKind::Runtime, ToolchainDomain::Java, &[], &["native-image"], &[], vec![
            tool("graalvm-native-image", "GraalVM Native Image", &["native-image"], "https://www.graalvm.org/latest/reference-manual/native-image/", "https://www.graalvm.org/latest/reference-manual/native-image/", "Install GraalVM and the Native Image tooling for the platform."),
        ]),
        lang("julia", "Julia", LanguageSupportKind::ProgrammingLanguage, ToolchainDomain::Julia, &["jl"], &["julia"], &[], vec![
            tool("julia", "Julia", &["julia"], "https://julialang.org/downloads/", "https://docs.julialang.org/", "Download Julia or install through the platform package manager."),
        ]),
        lang("r", "R", LanguageSupportKind::ProgrammingLanguage, ToolchainDomain::R, &["r", "R", "rmd", "qmd"], &["R"], &[], vec![
            tool("r-project", "R", &["R"], "https://www.r-project.org/", "https://cran.r-project.org/manuals.html", "Install R from CRAN or the platform package manager."),
        ]),
        lang("prolog", "Prolog", LanguageSupportKind::LanguageFamily, ToolchainDomain::Prolog, &["pl", "pro", "prolog"], &[], &["swipl", "gprolog"], vec![
            tool("swi-prolog", "SWI-Prolog", &["swipl"], "https://www.swi-prolog.org/", "https://www.swi-prolog.org/pldoc/doc/_SWI_/index.html", "Install SWI-Prolog through the platform package manager or official packages."),
            tool("gnu-prolog", "GNU Prolog", &["gprolog"], "https://www.gprolog.org/", "https://www.gprolog.org/manual/gprolog.html", "Install GNU Prolog through the platform package manager or from source."),
        ]),
        lang("lisp-scheme", "Lisp/Scheme", LanguageSupportKind::LanguageFamily, ToolchainDomain::Lisp, &["lisp", "lsp", "cl", "scm", "rkt"], &[], &["sbcl", "clisp", "ecl", "ccl", "racket", "guile", "chez"], vec![
            tool("sbcl", "SBCL", &["sbcl"], "https://sbcl.sourceforge.io/", "https://www.sbcl.org/manual/", "Install SBCL through the platform package manager or project binaries."),
            tool("clisp", "CLISP", &["clisp"], "https://clisp.sourceforge.io/", "https://clisp.sourceforge.io/impnotes/", "Install CLISP through the platform package manager where available."),
            tool("racket", "Racket", &["racket"], "https://racket-lang.org/", "https://docs.racket-lang.org/", "Download Racket or install through the platform package manager."),
            tool("guile", "Guile", &["guile"], "https://www.gnu.org/software/guile/", "https://www.gnu.org/software/guile/manual/", "Install GNU Guile through the platform package manager."),
        ]),
        lang("sql", "SQL", LanguageSupportKind::DatabaseDialect, ToolchainDomain::Database, &["sql"], &[], &["sqlite3", "psql", "mysql", "sqlcmd"], sql_tooling()),
        lang("plsql", "PL/SQL", LanguageSupportKind::DatabaseDialect, ToolchainDomain::Database, &["sql", "pls", "plb", "pks", "pkb"], &[], &["sqlplus", "sqlcl"], sql_tooling()),
        lang("tsql", "T-SQL", LanguageSupportKind::DatabaseDialect, ToolchainDomain::Database, &["sql"], &[], &["sqlcmd"], sql_tooling()),
        lang("plpgsql", "PL/pgSQL", LanguageSupportKind::DatabaseDialect, ToolchainDomain::Database, &["sql"], &[], &["psql"], sql_tooling()),
        lang("sqlite-sql", "SQLite SQL", LanguageSupportKind::DatabaseDialect, ToolchainDomain::Database, &["sql", "sqlite"], &["sqlite3"], &[], sql_tooling()),
        lang("html", "HTML", LanguageSupportKind::Markup, ToolchainDomain::Markup, &["html", "htm"], &[], &[], web_tooling()),
        lang("css", "CSS", LanguageSupportKind::Markup, ToolchainDomain::Markup, &["css"], &[], &[], web_tooling()),
        lang("scss-sass-less", "SCSS/Sass/Less", LanguageSupportKind::Markup, ToolchainDomain::Markup, &["scss", "sass", "less"], &[], &["node", "sass", "lessc"], web_tooling()),
        lang("xml-json-yaml-toml-markdown", "XML/JSON/YAML/TOML/Markdown/MDX", LanguageSupportKind::Markup, ToolchainDomain::Markup, &["xml", "json", "yaml", "yml", "toml", "md", "mdx"], &[], &[], web_tooling()),
        lang("javascript", "JavaScript", LanguageSupportKind::ProgrammingLanguage, ToolchainDomain::JavaScript, &["js", "mjs", "cjs"], &["node"], &[], javascript_tooling()),
        lang("typescript", "TypeScript", LanguageSupportKind::ProgrammingLanguage, ToolchainDomain::JavaScript, &["ts"], &["node", "npm"], &[], javascript_tooling()),
        lang("jsx-tsx", "JSX/TSX", LanguageSupportKind::ProgrammingLanguage, ToolchainDomain::JavaScript, &["jsx", "tsx"], &["node", "npm"], &[], javascript_tooling()),
        lang("flow", "Flow", LanguageSupportKind::ProgrammingLanguage, ToolchainDomain::JavaScript, &["js"], &["node"], &[], javascript_tooling()),
        lang("coffeescript", "CoffeeScript", LanguageSupportKind::ProgrammingLanguage, ToolchainDomain::JavaScript, &["coffee"], &["node", "npm"], &[], javascript_tooling()),
        lang("nodejs", "Node.js", LanguageSupportKind::Runtime, ToolchainDomain::JavaScript, &["js", "mjs", "cjs"], &["node", "npm"], &[], javascript_tooling()),
        lang("deno", "Deno", LanguageSupportKind::Runtime, ToolchainDomain::JavaScript, &["ts", "js"], &["deno"], &[], javascript_tooling()),
        lang("bun", "Bun", LanguageSupportKind::Runtime, ToolchainDomain::JavaScript, &["ts", "js"], &["bun"], &[], javascript_tooling()),
        js_framework("react", "React"),
        js_framework("preact", "Preact"),
        js_framework("vue", "Vue"),
        js_framework("svelte", "Svelte"),
        js_framework("angular", "Angular"),
        js_framework("solid", "Solid"),
        js_framework("qwik", "Qwik"),
        js_framework("lit", "Lit"),
        js_framework("astro", "Astro"),
        js_framework("nextjs", "Next.js"),
        js_framework("remix", "Remix"),
        js_framework("vite", "Vite"),
        js_framework("nuxt", "Nuxt"),
        js_framework("sveltekit", "SvelteKit"),
        js_framework("electron", "Electron"),
        js_framework("tauri-frontend", "Tauri Frontend"),
        js_framework("react-native", "React Native"),
        js_framework("expo", "Expo"),
        lang("go", "Go", LanguageSupportKind::ProgrammingLanguage, ToolchainDomain::Go, &["go"], &["go"], &[], vec![
            tool("go", "Go", &["go"], "https://go.dev/dl/", "https://go.dev/doc/", "Download Go or install through the platform package manager."),
        ]),
        lang("dotnet", ".NET Languages", LanguageSupportKind::LanguageFamily, ToolchainDomain::DotNet, &["cs", "fs", "vb"], &["dotnet"], &[], vec![
            tool("dotnet", ".NET SDK", &["dotnet"], "https://dotnet.microsoft.com/download", "https://learn.microsoft.com/dotnet/", "Install the .NET SDK for C#, F#, and Visual Basic projects."),
        ]),
        lang("ruby", "Ruby", LanguageSupportKind::ProgrammingLanguage, ToolchainDomain::Ruby, &["rb"], &["ruby"], &[], vec![
            tool("ruby", "Ruby", &["ruby"], "https://www.ruby-lang.org/en/downloads/", "https://docs.ruby-lang.org/", "Install Ruby through the platform package manager or Ruby installers."),
        ]),
        lang("lua", "Lua", LanguageSupportKind::ProgrammingLanguage, ToolchainDomain::Lua, &["lua"], &["lua"], &[], vec![
            tool("lua", "Lua", &["lua"], "https://www.lua.org/download.html", "https://www.lua.org/manual/", "Install Lua through the platform package manager or from source."),
        ]),
        lang("haskell", "Haskell", LanguageSupportKind::ProgrammingLanguage, ToolchainDomain::Functional, &["hs", "lhs"], &["ghc"], &[], vec![
            tool("ghc", "GHC", &["ghc"], "https://www.haskell.org/ghcup/", "https://downloads.haskell.org/ghc/latest/docs/users_guide/", "Install GHC and Cabal through GHCup."),
        ]),
        lang("erlang-elixir", "Erlang/Elixir", LanguageSupportKind::LanguageFamily, ToolchainDomain::Functional, &["erl", "hrl", "ex", "exs"], &[], &["erl", "elixir"], vec![
            tool("erlang", "Erlang/OTP", &["erl"], "https://www.erlang.org/downloads", "https://www.erlang.org/doc/", "Install Erlang/OTP through the platform package manager."),
            tool("elixir", "Elixir", &["elixir"], "https://elixir-lang.org/install.html", "https://hexdocs.pm/elixir/", "Install Elixir and Mix through the platform package manager."),
        ]),
        lang("dart-flutter", "Dart/Flutter", LanguageSupportKind::LanguageFamily, ToolchainDomain::JavaScript, &["dart"], &["dart"], &[], vec![
            tool("dart", "Dart", &["dart"], "https://dart.dev/get-dart", "https://dart.dev/guides", "Install Dart SDK or Flutter SDK."),
            tool("flutter", "Flutter", &["flutter"], "https://docs.flutter.dev/get-started/install", "https://docs.flutter.dev/", "Install Flutter SDK when mobile UI support is needed."),
        ]),
        lang("swift", "Swift", LanguageSupportKind::ProgrammingLanguage, ToolchainDomain::Swift, &["swift"], &["swift"], &[], vec![
            tool("swift", "Swift", &["swift"], "https://www.swift.org/install/", "https://docs.swift.org/swift-book/documentation/the-swift-programming-language/", "Install Swift toolchain for the platform."),
        ]),
        lang("shell", "Shell", LanguageSupportKind::ProgrammingLanguage, ToolchainDomain::Infrastructure, &["sh", "bash", "zsh", "ksh"], &[], &["sh", "bash", "zsh"], vec![
            tool("shell", "POSIX shell", &["sh"], "https://pubs.opengroup.org/onlinepubs/9699919799/utilities/V3_chap02.html", "https://pubs.opengroup.org/onlinepubs/9699919799/utilities/V3_chap02.html", "Use the system shell and optional shellcheck/shfmt profiles."),
        ]),
        lang("powershell", "PowerShell", LanguageSupportKind::ProgrammingLanguage, ToolchainDomain::Infrastructure, &["ps1", "psm1", "psd1"], &["pwsh"], &[], vec![
            tool("powershell", "PowerShell", &["pwsh"], "https://github.com/PowerShell/PowerShell", "https://learn.microsoft.com/powershell/", "Install PowerShell for the platform."),
        ]),
        lang("docker-compose", "Dockerfile/Compose", LanguageSupportKind::Infrastructure, ToolchainDomain::Infrastructure, &["Dockerfile", "dockerfile", "yaml", "yml"], &[], &["docker", "podman"], vec![
            tool("docker", "Docker", &["docker"], "https://docs.docker.com/get-docker/", "https://docs.docker.com/", "Install Docker or a compatible container runtime."),
            tool("podman", "Podman", &["podman"], "https://podman.io/", "https://docs.podman.io/", "Install Podman as a Docker-compatible runtime where preferred."),
        ]),
        lang("terraform-hcl", "Terraform HCL", LanguageSupportKind::Infrastructure, ToolchainDomain::Infrastructure, &["tf", "tfvars", "hcl"], &["terraform"], &[], vec![
            tool("terraform", "Terraform", &["terraform"], "https://developer.hashicorp.com/terraform/downloads", "https://developer.hashicorp.com/terraform/docs", "Install Terraform for HCL validation and plans."),
        ]),
        lang("kubernetes-helm", "Kubernetes YAML/Helm", LanguageSupportKind::Infrastructure, ToolchainDomain::Infrastructure, &["yaml", "yml", "tpl"], &[], &["kubectl", "helm"], vec![
            tool("kubectl", "kubectl", &["kubectl"], "https://kubernetes.io/docs/tasks/tools/", "https://kubernetes.io/docs/reference/kubectl/", "Install kubectl for cluster validation."),
            tool("helm", "Helm", &["helm"], "https://helm.sh/docs/intro/install/", "https://helm.sh/docs/", "Install Helm for chart support."),
        ]),
        lang("ansible", "Ansible", LanguageSupportKind::Infrastructure, ToolchainDomain::Infrastructure, &["yaml", "yml"], &["ansible"], &[], vec![
            tool("ansible", "Ansible", &["ansible"], "https://docs.ansible.com/ansible/latest/installation_guide/intro_installation.html", "https://docs.ansible.com/", "Install Ansible through pipx or the platform package manager."),
        ]),
        lang("nix", "Nix", LanguageSupportKind::Infrastructure, ToolchainDomain::Infrastructure, &["nix"], &["nix"], &[], vec![
            tool("nix", "Nix", &["nix"], "https://nixos.org/download/", "https://nixos.org/manual/nix/stable/", "Install Nix for Nix language and build support."),
        ]),
        lang("ci-cd-workflows", "CI/CD Workflows", LanguageSupportKind::Infrastructure, ToolchainDomain::Infrastructure, &["yaml", "yml", "Jenkinsfile"], &[], &[], vec![
            tool("workflow-linters", "Workflow linters", &[], "https://docs.github.com/actions", "https://docs.github.com/actions", "Use built-in YAML support plus optional provider-specific validators."),
        ]),
    ]
}

pub fn language_support_statuses_from_detected_ids(
    detected_executable_ids: &[String],
) -> Vec<LanguageSupportStatus> {
    let detected = detected_executable_ids
        .iter()
        .map(|value| value.as_str())
        .collect::<HashSet<_>>();
    supported_language_catalog()
        .iter()
        .map(|language| language.status_with(|executable| detected.contains(executable)))
        .collect()
}

pub fn detect_executables_on_path() -> Vec<String> {
    let candidates = supported_executable_ids();
    candidates
        .into_iter()
        .filter(|candidate| executable_on_path(candidate))
        .map(str::to_string)
        .collect()
}

pub fn supported_executable_ids() -> Vec<&'static str> {
    let mut ids = seed_executable_requirements()
        .iter()
        .map(|requirement| requirement.id)
        .collect::<Vec<_>>();
    for language in supported_language_catalog() {
        ids.extend(language.activation_all_of);
        ids.extend(language.activation_any_of);
        for tooling in language.tooling {
            ids.extend(tooling.executable_ids);
        }
    }
    ids.sort_unstable();
    ids.dedup();
    ids
}

pub fn language_tooling_install_flow_steps() -> &'static [&'static str] {
    &[
        "Detect language tooling from PATH, common SDK folders, version managers, and project wrappers.",
        "Show every supported language or framework as Active when required tooling is present.",
        "Show missing languages with Install Tooling and Configure Executable actions.",
        "Before download, test the selected tooling source URL.",
        "When the source is unreachable, open the network/proxy dialog in the same window.",
        "Let the user test direct connectivity, enter a proxy URL, and validate connectivity through the proxy.",
        "Persist the validated proxy setting in the user settings store.",
        "Download and install the selected open-source tooling using the platform installer strategy.",
        "Register the installed executable path and refresh language services for the editor.",
    ]
}

pub fn default_tooling_network_dialog() -> ToolingNetworkDialog {
    ToolingNetworkDialog {
        id: "setup.languageSupport.networkProxyDialog",
        title: "Tooling Download Network Settings",
        fields: vec![
            "tooling_source_url",
            "direct_connectivity_status",
            "proxy_url",
            "proxy_connectivity_status",
            "last_validation_message",
        ],
        direct_test_action: "setup.languageSupport.network.testDirect",
        proxy_test_action: "setup.languageSupport.network.testProxy",
        validation_success_action: "setup.languageSupport.network.saveValidatedProxy",
        test_targets: vec![
            NetworkTestTarget {
                id: "rustup",
                label: "Rust installer",
                url: "https://www.rust-lang.org/tools/install",
            },
            NetworkTestTarget {
                id: "nodejs",
                label: "Node.js downloads",
                url: "https://nodejs.org/",
            },
            NetworkTestTarget {
                id: "gnucobol",
                label: "GnuCOBOL",
                url: "https://gnucobol.sourceforge.io/",
            },
        ],
    }
}

pub fn validate_proxy_url(proxy_url: &str, reachable_through_proxy: bool) -> ProxyValidationState {
    let trimmed = proxy_url.trim();
    if trimmed.is_empty() {
        return ProxyValidationState::Empty;
    }
    if !(trimmed.starts_with("http://") || trimmed.starts_with("https://")) {
        return ProxyValidationState::InvalidUrl;
    }
    if reachable_through_proxy {
        ProxyValidationState::Reachable
    } else {
        ProxyValidationState::Unreachable
    }
}

fn executable_on_path(executable: &str) -> bool {
    let Some(path_value) = env::var_os("PATH") else {
        return false;
    };

    env::split_paths(&path_value).any(|directory| {
        executable_names_for_platform(executable)
            .iter()
            .any(|name| path_is_executable(&directory.join(name)))
    })
}

fn executable_names_for_platform(executable: &str) -> Vec<String> {
    if cfg!(windows) {
        vec![
            format!("{executable}.exe"),
            format!("{executable}.cmd"),
            format!("{executable}.bat"),
            executable.to_string(),
        ]
    } else {
        vec![executable.to_string()]
    }
}

fn path_is_executable(path: &Path) -> bool {
    fs::metadata(path)
        .map(|metadata| metadata.is_file())
        .unwrap_or(false)
}

fn lang(
    id: &'static str,
    display_name: &'static str,
    kind: LanguageSupportKind,
    domain: ToolchainDomain,
    file_extensions: &[&'static str],
    activation_all_of: &[&'static str],
    activation_any_of: &[&'static str],
    tooling: Vec<OpenSourceTooling>,
) -> ProgrammingLanguageSupport {
    let network_test_url = tooling
        .first()
        .map(|item| item.source_url)
        .unwrap_or("https://www.rust-lang.org/tools/install");
    ProgrammingLanguageSupport {
        id,
        display_name,
        kind,
        domain,
        file_extensions: file_extensions.to_vec(),
        activation_all_of: activation_all_of.to_vec(),
        activation_any_of: activation_any_of.to_vec(),
        tooling,
        install_action: "setup.languageSupport.installTooling",
        configure_action: "Settings > Languages & Toolchains > Configure Executable",
        network_test_url,
    }
}

fn tool(
    id: &'static str,
    display_name: &'static str,
    executable_ids: &[&'static str],
    source_url: &'static str,
    documentation_url: &'static str,
    install_strategy: &'static str,
) -> OpenSourceTooling {
    OpenSourceTooling {
        id,
        display_name,
        executable_ids: executable_ids.to_vec(),
        source_url,
        documentation_url,
        install_strategy,
    }
}

fn c_family_tooling() -> Vec<OpenSourceTooling> {
    vec![
        tool(
            "gcc",
            "GCC",
            &["gcc", "g++"],
            "https://gcc.gnu.org/",
            "https://gcc.gnu.org/onlinedocs/",
            "Install GCC through the platform package manager.",
        ),
        tool(
            "llvm-clang",
            "Clang/LLVM",
            &["clang", "clang++"],
            "https://llvm.org/",
            "https://clang.llvm.org/docs/",
            "Install LLVM/Clang through the platform package manager.",
        ),
        tool(
            "cmake",
            "CMake",
            &["cmake"],
            "https://cmake.org/",
            "https://cmake.org/documentation/",
            "Install CMake for project generation and builds.",
        ),
    ]
}

fn java_tooling() -> Vec<OpenSourceTooling> {
    vec![
        tool(
            "openjdk",
            "OpenJDK",
            &["java", "javac"],
            "https://openjdk.org/",
            "https://openjdk.org/guide/",
            "Install OpenJDK or a compatible JDK distribution.",
        ),
        tool(
            "maven",
            "Maven",
            &["mvn"],
            "https://maven.apache.org/",
            "https://maven.apache.org/guides/",
            "Install Maven for Java build support.",
        ),
        tool(
            "gradle",
            "Gradle",
            &["gradle"],
            "https://gradle.org/",
            "https://docs.gradle.org/",
            "Install Gradle or use a project Gradle wrapper.",
        ),
        tool(
            "graalvm",
            "GraalVM",
            &["native-image"],
            "https://www.graalvm.org/latest/reference-manual/native-image/",
            "https://www.graalvm.org/latest/reference-manual/native-image/",
            "Install GraalVM for Native Image profiles.",
        ),
    ]
}

fn sql_tooling() -> Vec<OpenSourceTooling> {
    vec![
        tool(
            "sqlite",
            "SQLite",
            &["sqlite3"],
            "https://www.sqlite.org/download.html",
            "https://www.sqlite.org/docs.html",
            "Install SQLite through one-click setup or the platform package manager.",
        ),
        tool(
            "postgresql",
            "PostgreSQL client",
            &["psql"],
            "https://www.postgresql.org/download/",
            "https://www.postgresql.org/docs/",
            "Install PostgreSQL client tools.",
        ),
        tool(
            "mysql",
            "MySQL client",
            &["mysql"],
            "https://dev.mysql.com/downloads/",
            "https://dev.mysql.com/doc/",
            "Install MySQL or MariaDB client tools.",
        ),
    ]
}

fn web_tooling() -> Vec<OpenSourceTooling> {
    vec![
        tool(
            "vscode-language-servers",
            "HTML/CSS/JSON language servers",
            &["node", "npm"],
            "https://github.com/hrsh7th/vscode-langservers-extracted",
            "https://github.com/hrsh7th/vscode-langservers-extracted",
            "Install language servers through npm when advanced diagnostics are required.",
        ),
        tool(
            "prettier",
            "Prettier",
            &["node", "npm"],
            "https://prettier.io/",
            "https://prettier.io/docs/en/",
            "Install Prettier through npm, pnpm, Yarn, or Bun.",
        ),
    ]
}

fn javascript_tooling() -> Vec<OpenSourceTooling> {
    vec![
        tool(
            "nodejs",
            "Node.js",
            &["node", "npm"],
            "https://nodejs.org/",
            "https://nodejs.org/api/",
            "Download Node.js or install through the platform package manager.",
        ),
        tool(
            "typescript",
            "TypeScript",
            &["node", "npm"],
            "https://www.typescriptlang.org/download/",
            "https://www.typescriptlang.org/docs/",
            "Install TypeScript through npm, pnpm, Yarn, or Bun.",
        ),
        tool(
            "deno",
            "Deno",
            &["deno"],
            "https://deno.com/",
            "https://docs.deno.com/",
            "Install Deno as an optional JavaScript/TypeScript runtime.",
        ),
        tool(
            "bun",
            "Bun",
            &["bun"],
            "https://bun.sh/",
            "https://bun.sh/docs",
            "Install Bun as an optional JavaScript runtime and package manager.",
        ),
    ]
}

fn js_framework(id: &'static str, display_name: &'static str) -> ProgrammingLanguageSupport {
    lang(
        id,
        display_name,
        LanguageSupportKind::Framework,
        ToolchainDomain::JavaScript,
        &["js", "jsx", "ts", "tsx"],
        &["node", "npm"],
        &[],
        javascript_tooling(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seed_requirements_include_sqlite_and_core_sdks() {
        let requirements = seed_executable_requirements();
        for id in [
            "cargo",
            "java",
            "node",
            "python",
            "sqlite3",
            "git",
            "cobc",
            "harbour",
            "swipl",
            "sbcl",
            "native-image",
        ] {
            assert!(requirements.iter().any(|requirement| requirement.id == id));
        }
    }

    #[test]
    fn missing_toolchain_profile_requires_configuration() {
        let profile = ToolchainProfile::missing("sqlite3");
        assert!(profile.requires_user_configuration());
    }

    #[test]
    fn language_catalog_covers_requested_and_common_missed_languages() {
        let catalog = supported_language_catalog();
        for id in [
            "assembly",
            "c",
            "cpp",
            "objective-c",
            "ada",
            "python",
            "php",
            "perl",
            "java",
            "julia",
            "rust",
            "cobol",
            "basic",
            "clipper-xbase",
            "html",
            "css",
            "graalvm-native-image",
            "nodejs",
            "react",
            "prolog",
            "lisp-scheme",
            "r",
            "sql",
            "plsql",
            "tsql",
            "go",
            "dotnet",
        ] {
            assert!(catalog.iter().any(|language| language.id == id), "{id}");
        }
        assert!(catalog.len() >= 60);
    }

    #[test]
    fn active_status_requires_detected_tooling() {
        let detected = vec![
            "rustc".to_string(),
            "cargo".to_string(),
            "node".to_string(),
            "npm".to_string(),
            "sqlite3".to_string(),
        ];
        let statuses = language_support_statuses_from_detected_ids(&detected);
        assert!(statuses
            .iter()
            .any(|status| status.language_id == "rust" && status.is_active()));
        assert!(statuses
            .iter()
            .any(|status| status.language_id == "react" && status.is_active()));
        assert!(statuses.iter().any(|status| status.language_id == "cobol"
            && status.state == LanguageSupportState::MissingTooling
            && status.missing_required_executables.contains(&"cobc")));
    }

    #[test]
    fn markup_without_external_tooling_is_active() {
        let statuses = language_support_statuses_from_detected_ids(&[]);
        assert!(statuses
            .iter()
            .any(|status| status.language_id == "html" && status.is_active()));
        assert!(statuses
            .iter()
            .any(|status| status.language_id == "css" && status.is_active()));
    }

    #[test]
    fn network_dialog_supports_direct_and_proxy_validation() {
        let dialog = default_tooling_network_dialog();
        assert!(dialog.fields.contains(&"proxy_url"));
        assert!(dialog
            .test_targets
            .iter()
            .any(|target| target.id == "gnucobol"));
        assert_eq!(
            validate_proxy_url("http://proxy.example:8080", true),
            ProxyValidationState::Reachable
        );
        assert_eq!(
            validate_proxy_url("socks5://proxy.example:1080", true),
            ProxyValidationState::InvalidUrl
        );
        assert!(language_tooling_install_flow_steps()
            .iter()
            .any(|step| step.contains("validate connectivity through the proxy")));
    }
}
