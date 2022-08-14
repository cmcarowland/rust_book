use std::fmt;
use std::io;
use std::io::Write;
use std::fs;

struct Account {
    first_name: String,
    last_name: String,
    password: String,
    email: String,
}

impl Account {
    fn new() -> Self {
        let first_name = String::new();
        let last_name = String::new();
        let password = String::new();
        let email = String::new();

        Account {first_name, last_name, password, email}
    }
    
    fn from_string(in_data: &str) -> Self {
        let data: Vec<&str> = in_data.split("\t").collect::<Vec<&str>>();
        let first_name = data[0].to_string();
        let last_name = data[1].to_string();
        let password = data[2].to_string();
        let email = data[3].to_string();

        Account {first_name, last_name, password, email}
    }

    fn set_first(&mut self, name: &String) {
        self.first_name = name.trim().to_string();
    }
    
    fn set_last(&mut self, name: &String) {
        self.last_name = name.trim().to_string();
    }
    
    fn set_password(&mut self, name: &String) {
        self.password = name.trim().to_string();
    }
    
    fn set_email(&mut self, name: &String) {
        self.email = name.trim().to_string();
    }

    fn to_file(&self) -> String {
        let mut outStr = String::new();

        outStr += &self.first_name;
        outStr += "\t";
        outStr += &self.last_name;
        outStr += "\t";
        outStr += &self.password;
        outStr += "\t";
        outStr += &self.email;
        outStr += "\n";

        outStr
    }
}

impl fmt::Display for Account {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut nameString: String = self.first_name.clone();
        nameString += " ";
        nameString += &self.last_name[..];
        write!(f, "{:20} {:20}", nameString, self.email)
    }
}

fn main() {
    let mut accounts = Vec::new();
    read_accounts_from_file(&mut accounts);
    display_accounts(&accounts);

    loop {
        let account = CreateNewUser();
        if accounts.iter().position(|x| x.email == account.email) != None {
            println!("{} already exists - account not added.\n", account.email);
            if !check_continue() {
                break;
            }
            continue;
        }

        println!("{} was added for {} {}\n", account.email, account.first_name, account.last_name);
        accounts.push(account);
        write_accounts_to_file(&accounts);
        display_accounts(&accounts);
        if !check_continue() {
            break;
        }
    }
}

fn check_continue() -> bool {
    let mut buf = String::new();
    print!("Continue (y/n) :");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buf).unwrap();

    match buf.trim() {
        "n" => {
            false
        }
        _ => {
            true
        }
    }
}

fn write_accounts_to_file(accounts: &Vec<Account>) {
    let mut outFile = fs::File::create("Accounts.txt").unwrap();

    for account in accounts {
        outFile.write_all(&account.to_file().as_bytes());
    }
}

fn read_accounts_from_file(accounts: &mut Vec<Account>) {
    accounts.clear();
    let contents = fs::read_to_string("Accounts.txt").unwrap();

    for line in contents.lines() {
        accounts.push(Account::from_string(line));
    }
}

fn display_accounts(accounts: &Vec<Account>) {
    println!("{:20} {:20}", "Name", "Email");
    for account in accounts {
        println!("{}", account);
    }

    println!("");
}

fn get_user_string_input(message: &str, buf: &mut String)  {
    print!("Enter {}", message);
    io::stdout().flush().unwrap();
    io::stdin().read_line(buf).unwrap();
}

fn CreateNewUser() -> Account {
    let mut account = Account::new();
    let mut buf = String::new();

    get_user_string_input("First Name : ", &mut buf);
    account.set_first(&buf);
    buf.clear();

    get_user_string_input("Last Name : ", &mut buf);
    account.set_last(&buf);
    buf.clear();
    
    get_user_string_input("Password : ", &mut buf);
    account.set_password(&buf);
    buf.clear();
    
    get_user_string_input("Email : ", &mut buf);
    account.set_email(&buf);
    buf.clear();

    account
}