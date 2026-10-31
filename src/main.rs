use std::{env, mem};


fn chec_is_sign(symbol:char) -> bool {
    if symbol == '+' || symbol == '-' || symbol == '*' || symbol == '/' {
        return true;
    } 
    return false;
}

// считает выражения с 1 действием
fn arithmetic(math:String) -> Result<String, ()> { 
    fn split_math(math:String) -> Result<(i32, i32, char), ()> {
        let num1: i32;
        let num2: i32;
        let mut operator_pos: usize = 0;
        let s_m: Vec<char> = math.chars().collect();
 
        for sinx in 0..s_m.len() {
            if chec_is_sign(s_m[sinx]) && sinx > 0 && !chec_is_sign(s_m[sinx - 1]) {
                operator_pos = sinx;
                break;
            }
        }
        
        let mut buffer: String = String::new();
        for i in 0..operator_pos { buffer += &s_m[i].to_string(); }
        
        match buffer.parse::<i32>() {
            Ok(num) => num1 = num,
            Err(err) => { 
                eprintln!("{}", err);
                return Err(());
            }
        }
        
        buffer = String::new();
        for i in operator_pos+1..s_m.len() { buffer += &s_m[i].to_string(); }

        match buffer.parse::<i32>() {
            Ok(num) => num2 = num,
            Err(err) => { 
                eprintln!("{}", err);
                return Err(());
            }
        }
        
        return Ok((num1, num2, s_m[operator_pos]));
    }

    match math.parse::<i32>() {
        Ok(integ) => return Ok(integ.to_string()),
        Err(_) => { }
    }    

    let num1:i32;
    let num2:i32;
    let operator:char;
    
    match split_math(math.clone()) {
        Ok((_num1, _num2, _operator)) => {
            num1 = _num1;
            num2 = _num2;
            operator = _operator;
        },
        Err(_) => return Err(())
    }
        
    if operator == '+' {
        return Ok((num1 + num2).to_string());

    } else if operator == '-' {
        return Ok((num1 - num2).to_string());

    } else if operator == '*' {
        return Ok((num1 * num2).to_string());

    }  else if operator == '/' {
        return Ok((num1 / num2).to_string());

    } else {
        eprintln!("incorrect: {}", math);
        return Err(());
    }
}

// считвает сложные выражения с множеством действий
fn calc(mut aric:String) -> Result<i32, ()> { // проблема с отрицаельными числами тут !!!
    fn rasbor(aric:String, operation:char) -> Result<String, ()>{
        let mut pos_pp: usize = 0;
        let mut pos_oper: usize = 0;
        let m_a: Vec<char> = aric.chars().collect();
        let mut buffer: String = String::new();

        for inx in 0..m_a.len() {
            if m_a[inx] == operation {
                pos_oper = inx;
                break;
            }   
            if chec_is_sign(m_a[inx]) && inx > 0 && !chec_is_sign(m_a[inx - 1]) { pos_pp = inx+1; }
        }
        //println!("{}", m_a[pos_oper]); 

        let mut inx:usize;
        for inx in pos_pp..pos_oper {
            if chec_is_sign(m_a[inx]) && inx > 0 && !chec_is_sign(m_a[inx - 1]) { break; }
            buffer += &m_a[inx].to_string();
        }

        inx = pos_oper.into();
        while inx < m_a.len() {
            if chec_is_sign(m_a[inx]) && inx != pos_oper && inx > 0 && !chec_is_sign(m_a[inx - 1]) { break; }
            buffer += &m_a[inx].to_string();

            inx +=1;
        }

        let num: String;
        match arithmetic(buffer.clone()) {
            Ok(result) => {
                num = result;
            },
            Err(_) => {
                eprintln!("error calculation {}", buffer);
                return Err(());
            }
        }
        
        return Ok(str::replace(&aric, &mem::take(&mut buffer), &num));
    }

    if !(aric.contains("+") || aric.contains("-") || aric.contains("/") || aric.contains("*")){
        match aric.parse::<i32>() {
            Ok(integ) => return Ok(integ),
            Err(error) => {
                println!("error({}) operation in {}", error, aric);
                return Err(());
            }
        }
    }
   //как будто проблема где то тут
    while aric.contains("+") || aric.contains("-") || aric.contains("/") || aric.contains("*") {
        match aric.parse::<i32>() { 
            Ok(integ) => return Ok(integ),
            Err(_) => { }
        }    

        if aric.contains("*") || aric.contains("/") {
            if aric.contains("*"){      
                match rasbor(aric, '*') {
                    Ok(relust) => aric = relust,
                    Err(_) => return Err(())
                }
            }
            if aric.contains("/"){ 
                match rasbor(aric, '/') {
                    Ok(relust) => aric = relust,
                    Err(_) => return Err(())
                }
            }
        } else if aric.contains("+") || aric.contains("-") {
            if aric.contains("+"){ 
                match rasbor(aric, '+') {
                    Ok(relust) => aric = relust,
                    Err(_) => return Err(())
                }
            }
            if aric.contains("-"){ 
                match rasbor(aric, '-') {
                    Ok(relust) => aric = relust,
                    Err(_) => return Err(())
                }
            }
        }
    } 

    match aric.parse::<i32>() {
        Ok(integ) => return Ok(integ),
        Err(error) => {
            println!("error({}) operation in {}", error, aric);
            return Err(());
        }
    }     
}

fn parser(text:String) -> String {
    let mut buffer: String = String::new();
    let mut arifmetic_buff: String = String::new();
    let mut open_flag: bool = false;

    // проверка наличия скобок
    if text.contains("(") && text.contains(")") {
        for tt in text.chars() { 
            if tt == '(' { open_flag = true; }
            if tt == ')' { open_flag= false; }
            if !open_flag {
                if tt != ')' {
                    buffer += &tt.to_string();
                } else {
                    buffer += &calc(mem::take(&mut arifmetic_buff)).unwrap().to_string();
                }
            } else {
                if tt != '(' {
                    arifmetic_buff += &tt.to_string();
                }
            }
        }
    } else { buffer = text; }
    
    
    return calc(mem::take(&mut buffer)).unwrap().to_string();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let out: String;

    // Проверяем, есть ли аргументы
    if args.len() <= 1 {
        println!("using:\ntcalc [equation] [args]");
    } else {
        let data:String = str::replace(&args[1], " ", "").to_string();
        
        out = parser(data);
        
        //println!("arithmetic: {}", arithmetic("-3--1".to_string()).unwrap());
        println!("{}", out);
    }
}
