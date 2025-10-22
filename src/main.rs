use git2::Repository;
use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check if a path argument was provided
    if args.len() < 2 {
        eprintln!("Usage: {} <path>", args[0]);
        std::process::exit(1);
    }

    //
    // Get the path from the command-line argument

    let true_path = PathBuf::from(&args[1]);
    let _repo = match Repository::init(&true_path) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to init: {}", e),
    };
    //println!("{:#?}", true_path);

    let vale_ini_contents = String::from(
        r#"
	      MinAlertLevel = suggestion

	      [*]
	      BasedOnStyles = Vale
    "#,
    );
    match write_vale_ini(&true_path, &vale_ini_contents) {
        Ok(_) => {}
        Err(_) => {}
    };
    let tex_init_contents: String = String::from(
        r#"
            \documentclass[11pt,reqno]{article}

            \usepackage{amscd}
            \usepackage{amsfonts}
            \usepackage{amsmath}
            \usepackage{amssymb}
            \usepackage{amsthm}
            \usepackage{fancyhdr}
            \usepackage{latexsym}
            \usepackage[colorlinks=true, pdfstartview=FitV, linkcolor=blue,
                        citecolor=blue, urlcolor=blue]{hyperref}
            \usepackage[pdftex]{graphicx}
            \usepackage{epstopdf}
            
            % Any custom macros or packages can go here
            \newtheorem{remark}{Remark}
            
            \title{\textbf{Title}}
            \author{Anthony Ozog }
            \date{\today}
            \markboth{title}{A. Ozog}
            
            \begin{document}
            
            \maketitle
            
            
            \end{document}
       "#,
    );
    match write_main_tex(&true_path, &tex_init_contents) {
        Ok(_) => {}
        Err(_) => {}
    };

    let _cmd_out = Command::new("pdflatex")
        .arg("main.tex")
        .current_dir(true_path)
        .output()
        .expect("pdflatex failed");
}

fn write_vale_ini(path: &PathBuf, content: &str) -> io::Result<()> {
    let file_name = Path::new("vale.ini");
    let mut file = File::create(path.join(file_name))?; // Creates the file, overwriting if it exists
    file.write_all(content.as_bytes())?; // Writes the content to the file
    Ok(())
}

fn write_main_tex(path: &PathBuf, content: &str) -> io::Result<()> {
    let file_name = Path::new("main.tex");
    let mut file = File::create(path.join(file_name))?; // Creates the file, overwriting if it exists
    file.write_all(content.as_bytes())?; // Writes the content to the file
    Ok(())
}
