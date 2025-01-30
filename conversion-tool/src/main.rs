use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        println!("Select an option:");
        println!("1. Calculate Corrected Reading");
        println!("2. Convert Unit Measurements");

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim().parse::<u32>() {
            Ok(choice) => match choice {
                1 => calculate_corrected_reading()?,
                2 => convert_units()?,
                _ => println!("Invalid choice. Please enter 1 or 2."),
            },
            Err(_) => println!("Invalid input. Please enter a number."),
        }

        // Ask if the user wants to perform another calculation/conversion
        println!("Do you want to perform another operation? (y/n):");
        let mut continue_input = String::new();
        io::stdin().read_line(&mut continue_input)?;
        if continue_input.trim().to_lowercase() != "y" {
            break; // Exit the loop if the user doesn't enter 'y'
        }
    }
    Ok(())
}

fn calculate_corrected_reading() -> Result<(), Box<dyn std::error::Error>> {
    const ATMOSPHERIC_PRESSURE_CONSTANT: f64 = 14.43;
    const BASE_PRESSURE_CONSTANT: f64 = 14.73;

    let pressure_psi: f64 = loop {  // Declare and get pressure input HERE
        println!("Enter the pressure at the meter (P) in psi:");
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        match input.trim().parse() {
            Ok(value) if value >= 0.0 => break value,
            _ => println!("Invalid input. Please enter a non-negative number."),
        }
    };

    let uncorrected_reading: f64 = loop { //Uncorrected reading input
        println!("Enter the uncorrected meter reading (U):");
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        match input.trim().parse() {
            Ok(value) if value >= 0.0 => break value,
            _ => println!("Invalid input. Please enter a non-negative number."),
        }
    };

    let unit: String = loop { //Unit input
        println!("Select the unit for U:");
        println!("1. CF");
        println!("2. CCF");
        println!("3. MCF");

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim().parse::<u32>() {
            Ok(choice) => match choice {
                1 => break "CF".to_string(),
                2 => break "CCF".to_string(),
                3 => break "MCF".to_string(),
                _ => println!("Invalid choice. Please enter 1, 2, or 3."),
            },
            Err(_) => println!("Invalid input. Please enter a number."),
        }
    };

    let uncorrected_cf = match unit.as_str() {
        "CF" => uncorrected_reading,
        "CCF" => uncorrected_reading * 100.0,
        "MCF" => uncorrected_reading * 1000.0,
        _ => unreachable!(),
    };

    let pressure_factor = (pressure_psi + ATMOSPHERIC_PRESSURE_CONSTANT) / BASE_PRESSURE_CONSTANT;
    let corrected_cf = pressure_factor * uncorrected_cf;
    let corrected_ccf = corrected_cf / 100.0;
    let corrected_mcf = corrected_cf / 1000.0;

    println!("Pressure Factor (PF): {:.6}", pressure_factor);
    println!("Corrected Reading (C):");
    println!("  CF:  {}", corrected_cf as i64);
    println!("  CCF: {}", corrected_ccf as i64);
    println!("  MCF: {}", corrected_mcf as i64);

    Ok(())
}

fn convert_units() -> Result<(), Box<dyn std::error::Error>> {
    println!("Select the unit to convert from:");
    println!("1. BTU");
    println!("2. MBTU");
    println!("3. MMBTU");
    println!("4. CF");
    println!("5. CCF");
    println!("6. MCF");

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let from_unit = match input.trim().parse::<u32>() {
        Ok(choice) if choice >= 1 && choice <= 6 => choice,
        _ => {
            println!("Invalid choice. Please enter a number between 1 and 6.");
            return Ok(());
        }
    };

    println!("Enter the value to convert:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let value: f64 = match input.trim().parse() {
        Ok(val) => val,
        Err(_) => {
            println!("Invalid input. Please enter a number.");
            return Ok(());
        }
    };

    let (btu, mbtu, mmbtu, cf, ccf, mcf) = match from_unit {
        1 => (value, value / 1000.0, value / 1000000.0, value * 0.0009634, value * 0.09634, value * 0.9634), //BTU
        2 => (value * 1000.0, value, value / 1000.0, value * 0.9634, value * 96.34, value * 963.4), //MBTU
        3 => (value * 1000000.0, value * 1000.0, value, value * 963.4, value * 96340.0, value * 963400.0), //MMBTU
        4 => (value * 1038.0, value * 1.038, value * 0.001038, value, value / 100.0, value / 1000.0), //CF (Corrected)
        5 => (value * 103800.0, value * 103.8, value * 0.1038, value * 100.0, value, value / 10.0), //CCF (Corrected)
        6 => (value * 1038000.0, value * 1038.0, value * 1.038, value * 1000.0, value * 10.0, value), //MCF (Corrected)
        _ => unreachable!(),
    };

    println!("Converted values:");

    // Formatting with conditional decimal places
    println!("BTU: {}", format_value(btu));
    println!("MBTU: {}", format_value(mbtu));
    println!("MMBTU: {}", format_value(mmbtu));
    println!("CF: {}", format_value(cf));
    println!("CCF: {}", format_value(ccf));
    println!("MCF: {}", format_value(mcf));

    Ok(())
}

fn format_value(value: f64) -> String {
    let s = format!("{:.6}", value); // Format to 6 decimal places initially
    let trimmed = s.trim_end_matches('0').trim_end_matches('.'); // Remove trailing zeros and .
    if trimmed.is_empty() {
        "0".to_string() // Display 0 if value is effectively zero
    } else {
        trimmed.to_string()
    }
}

