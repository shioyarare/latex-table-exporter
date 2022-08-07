use std::{env, cmp};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    // ファイル名が含まれていない場合終了
    if args.len() < 2 { panic!("No input file"); }
    
    let mut filename: String = args[1].clone();
    let c = filename.chars().nth(0).unwrap();
    if c != '/' && c != '~' {
        // フルパスで指定されていない場合はフルパスを作成
        let path = env::current_dir()?;
        filename = path.to_str().unwrap().to_string() + "/" + filename.as_str();
    }

    println!("{}", filename);
    let mut rows_len: usize = 0;  // 最大列数
    let mut columns: Vec<Vec<String>> = Vec::new(); // 行データ 

    for result in BufReader::new(File::open(filename)?).lines() {
        // 一行ずつ読み取り分割
        let l: Vec<String> = result.unwrap()
                                       .as_str()
                                       .trim()
                                       .split_whitespace()
                                       .map(|x| x.to_string() )
                                       .collect();
        // 値の更新
        rows_len = cmp::max(rows_len, l.len());
        columns.push(l);
    }

    // 表の出力処理
    println!("");
    println!("\\begin{{table}}{{H}}");
    println!("  \\caption{{title}}");
    println!("  \\label{{tab:data}}");
    println!("  \\centering");
    println!("  \\begin{{tabular}}{{{}}}", (0..rows_len).map(|_| "c").collect::<String>());
    println!("    \\hline");

    for (i, col) in columns.iter().enumerate() {
        let deficit_num: usize = cmp::max(0, rows_len - col.len());
        let add_str: String = (0..deficit_num).map(|_| "& ").collect::<String>();
        println!("    {} {}\\\\", col.join(" & "), add_str);
        if i == 0 { println!("    \\hline \\hline"); }
    }
    println!("    \\hline");
    println!("  \\end{{tabular}}");
    println!("\\end{{table}}");
    println!("");

    Ok(())
}
