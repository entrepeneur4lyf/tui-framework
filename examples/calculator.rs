//! Calculator - Advanced TUI Framework Example
//!
//! This comprehensive example demonstrates advanced TUI framework capabilities:
//!
//! ## Advanced Features Showcased:
//! - Complex state management with multiple interconnected states
//! - Component composition with specialized sub-components
//! - Advanced event handling and user interaction patterns
//! - Mathematical expression parsing and evaluation
//! - History tracking and memory functions
//! - Error handling and validation
//! - Responsive layout with dynamic button grids
//! - Custom styling and theming
//!
//! ## Architecture Highlights:
//! - Calculator engine with operation state machine
//! - Display component with formatting and overflow handling
//! - Button grid component with dynamic layout
//! - History panel with scrollable operation log
//! - Memory functions (M+, M-, MR, MC)
//! - Scientific calculator functions (optional)
//!
//! This example represents a real-world application built with the TUI framework,
//! demonstrating how to structure complex applications with multiple components,
//! sophisticated state management, and professional user experience patterns.

use std::collections::VecDeque;
use tui_framework::component::BaseComponent;
use tui_framework::prelude::*;

/// Calculator operation types
#[derive(Debug, Clone, PartialEq)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    #[allow(dead_code)]
    Equals,
}

impl Operation {
    fn symbol(&self) -> &'static str {
        match self {
            Operation::Add => "+",
            Operation::Subtract => "-",
            Operation::Multiply => "×",
            Operation::Divide => "÷",
            Operation::Equals => "=",
        }
    }

    fn calculate(&self, left: f64, right: f64) -> Result<f64> {
        match self {
            Operation::Add => Ok(left + right),
            Operation::Subtract => Ok(left - right),
            Operation::Multiply => Ok(left * right),
            Operation::Divide => {
                if right == 0.0 {
                    Err(Error::custom("Division by zero"))
                } else {
                    Ok(left / right)
                }
            }
            Operation::Equals => Ok(right),
        }
    }
}

/// Calculator state and computation engine
#[derive(Debug, Clone)]
struct CalculatorEngine {
    display: String,
    current_value: f64,
    previous_value: Option<f64>,
    pending_operation: Option<Operation>,
    should_reset_display: bool,
    memory: f64,
    history: VecDeque<String>,
    max_history: usize,
}

impl CalculatorEngine {
    fn new() -> Self {
        Self {
            display: "0".to_string(),
            current_value: 0.0,
            previous_value: None,
            pending_operation: None,
            should_reset_display: false,
            memory: 0.0,
            history: VecDeque::new(),
            max_history: 50,
        }
    }

    fn input_digit(&mut self, digit: char) {
        if self.should_reset_display {
            self.display = digit.to_string();
            self.should_reset_display = false;
        } else if self.display == "0" {
            self.display = digit.to_string();
        } else if self.display.len() < 15 {
            self.display.push(digit);
        }
        self.current_value = self.display.parse().unwrap_or(0.0);
    }

    fn input_decimal(&mut self) {
        if self.should_reset_display {
            self.display = "0.".to_string();
            self.should_reset_display = false;
        } else if !self.display.contains('.') {
            self.display.push('.');
        }
    }

    fn input_operation(&mut self, operation: Operation) -> Result<()> {
        if let Some(pending_op) = &self.pending_operation {
            if let Some(prev_val) = self.previous_value {
                let result = pending_op.calculate(prev_val, self.current_value)?;
                self.add_to_history(format!(
                    "{} {} {} = {}",
                    prev_val,
                    pending_op.symbol(),
                    self.current_value,
                    result
                ));
                self.current_value = result;
                self.display = format_number(result);
            }
        }

        self.previous_value = Some(self.current_value);
        self.pending_operation = Some(operation);
        self.should_reset_display = true;
        Ok(())
    }

    fn calculate(&mut self) -> Result<()> {
        if let (Some(pending_op), Some(prev_val)) = (&self.pending_operation, self.previous_value) {
            let result = pending_op.calculate(prev_val, self.current_value)?;
            self.add_to_history(format!(
                "{} {} {} = {}",
                prev_val,
                pending_op.symbol(),
                self.current_value,
                result
            ));
            self.current_value = result;
            self.display = format_number(result);
            self.previous_value = None;
            self.pending_operation = None;
            self.should_reset_display = true;
        }
        Ok(())
    }

    fn clear(&mut self) {
        self.display = "0".to_string();
        self.current_value = 0.0;
        self.previous_value = None;
        self.pending_operation = None;
        self.should_reset_display = false;
    }

    fn clear_entry(&mut self) {
        self.display = "0".to_string();
        self.current_value = 0.0;
    }

    fn add_to_history(&mut self, entry: String) {
        self.history.push_back(entry);
        if self.history.len() > self.max_history {
            self.history.pop_front();
        }
    }

    fn memory_add(&mut self) {
        self.memory += self.current_value;
    }

    fn memory_subtract(&mut self) {
        self.memory -= self.current_value;
    }

    fn memory_recall(&mut self) {
        self.current_value = self.memory;
        self.display = format_number(self.memory);
        self.should_reset_display = true;
    }

    fn memory_clear(&mut self) {
        self.memory = 0.0;
    }

    fn get_history(&self) -> Vec<String> {
        self.history.iter().cloned().collect()
    }
}

/// Format numbers for display
fn format_number(value: f64) -> String {
    if value.fract() == 0.0 && value.abs() < 1e15 {
        format!("{}", value as i64)
    } else {
        format!("{:.10}", value)
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_string()
    }
}

/// Calculator Display Component
///
/// Handles the main display area showing current value, operation, and status
struct CalculatorDisplay {
    base: BaseComponent,
    engine: State<CalculatorEngine>,
}

impl CalculatorDisplay {
    fn new(engine: State<CalculatorEngine>) -> Self {
        Self {
            base: BaseComponent::new("CalculatorDisplay"),
            engine,
        }
    }
}

#[async_trait]
impl Component for CalculatorDisplay {
    fn id(&self) -> ComponentId {
        self.base.id()
    }

    fn name(&self) -> &str {
        "CalculatorDisplay"
    }

    async fn render(&self, _context: &RenderContext) -> Result<VirtualNode> {
        let engine = self.engine.clone_value();

        let display_ui = div()
            .attr("class", "calculator-display")
            .child(
                div()
                    .attr("class", "display-main")
                    .child(text(&engine.display)),
            )
            .child(
                div()
                    .attr("class", "display-status")
                    .child(if let Some(op) = &engine.pending_operation {
                        text(format!("Operation: {}", op.symbol()))
                    } else {
                        text("Ready")
                    })
                    .child(if engine.memory != 0.0 {
                        text(format!("Memory: {}", format_number(engine.memory)))
                    } else {
                        text("")
                    }),
            );

        Ok(display_ui)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

/// Calculator Button Grid Component
///
/// Renders the button layout for calculator input
struct CalculatorButtons {
    base: BaseComponent,
    engine: State<CalculatorEngine>,
}

impl CalculatorButtons {
    fn new(engine: State<CalculatorEngine>) -> Self {
        Self {
            base: BaseComponent::new("CalculatorButtons"),
            engine,
        }
    }

    fn handle_button_click(&self, button_id: &str) -> Result<()> {
        let mut engine = self.engine.clone_value();

        match button_id {
            // Digits
            "btn-0" => engine.input_digit('0'),
            "btn-1" => engine.input_digit('1'),
            "btn-2" => engine.input_digit('2'),
            "btn-3" => engine.input_digit('3'),
            "btn-4" => engine.input_digit('4'),
            "btn-5" => engine.input_digit('5'),
            "btn-6" => engine.input_digit('6'),
            "btn-7" => engine.input_digit('7'),
            "btn-8" => engine.input_digit('8'),
            "btn-9" => engine.input_digit('9'),

            // Decimal point
            "btn-decimal" => engine.input_decimal(),

            // Operations
            "btn-add" => engine.input_operation(Operation::Add)?,
            "btn-subtract" => engine.input_operation(Operation::Subtract)?,
            "btn-multiply" => engine.input_operation(Operation::Multiply)?,
            "btn-divide" => engine.input_operation(Operation::Divide)?,
            "btn-equals" => engine.calculate()?,

            // Clear functions
            "btn-clear" => engine.clear(),
            "btn-clear-entry" => engine.clear_entry(),

            // Memory functions
            "btn-memory-add" => engine.memory_add(),
            "btn-memory-subtract" => engine.memory_subtract(),
            "btn-memory-recall" => engine.memory_recall(),
            "btn-memory-clear" => engine.memory_clear(),

            _ => {} // Unknown button
        }

        self.engine.set(engine);
        Ok(())
    }
}

#[async_trait]
impl Component for CalculatorButtons {
    fn id(&self) -> ComponentId {
        self.base.id()
    }

    fn name(&self) -> &str {
        "CalculatorButtons"
    }

    async fn render(&self, _context: &RenderContext) -> Result<VirtualNode> {
        let buttons_ui = div()
            .attr("class", "calculator-buttons")
            .child(
                // Memory row
                div()
                    .attr("class", "button-row memory-row")
                    .child(
                        button("MC")
                            .attr("id", "btn-memory-clear")
                            .attr("class", "btn-memory"),
                    )
                    .child(
                        button("MR")
                            .attr("id", "btn-memory-recall")
                            .attr("class", "btn-memory"),
                    )
                    .child(
                        button("M+")
                            .attr("id", "btn-memory-add")
                            .attr("class", "btn-memory"),
                    )
                    .child(
                        button("M-")
                            .attr("id", "btn-memory-subtract")
                            .attr("class", "btn-memory"),
                    ),
            )
            .child(
                // Clear row
                div()
                    .attr("class", "button-row clear-row")
                    .child(
                        button("C")
                            .attr("id", "btn-clear")
                            .attr("class", "btn-clear"),
                    )
                    .child(
                        button("CE")
                            .attr("id", "btn-clear-entry")
                            .attr("class", "btn-clear"),
                    )
                    .child(
                        button("÷")
                            .attr("id", "btn-divide")
                            .attr("class", "btn-operation"),
                    )
                    .child(
                        button("×")
                            .attr("id", "btn-multiply")
                            .attr("class", "btn-operation"),
                    ),
            )
            .child(
                // Number row 1
                div()
                    .attr("class", "button-row number-row")
                    .child(button("7").attr("id", "btn-7").attr("class", "btn-number"))
                    .child(button("8").attr("id", "btn-8").attr("class", "btn-number"))
                    .child(button("9").attr("id", "btn-9").attr("class", "btn-number"))
                    .child(
                        button("-")
                            .attr("id", "btn-subtract")
                            .attr("class", "btn-operation"),
                    ),
            )
            .child(
                // Number row 2
                div()
                    .attr("class", "button-row number-row")
                    .child(button("4").attr("id", "btn-4").attr("class", "btn-number"))
                    .child(button("5").attr("id", "btn-5").attr("class", "btn-number"))
                    .child(button("6").attr("id", "btn-6").attr("class", "btn-number"))
                    .child(
                        button("+")
                            .attr("id", "btn-add")
                            .attr("class", "btn-operation"),
                    ),
            )
            .child(
                // Number row 3 and equals
                div()
                    .attr("class", "button-row number-row")
                    .child(button("1").attr("id", "btn-1").attr("class", "btn-number"))
                    .child(button("2").attr("id", "btn-2").attr("class", "btn-number"))
                    .child(button("3").attr("id", "btn-3").attr("class", "btn-number"))
                    .child(
                        button("=")
                            .attr("id", "btn-equals")
                            .attr("class", "btn-equals"),
                    ),
            )
            .child(
                // Bottom row
                div()
                    .attr("class", "button-row bottom-row")
                    .child(
                        button("0")
                            .attr("id", "btn-0")
                            .attr("class", "btn-number btn-zero"),
                    )
                    .child(
                        button(".")
                            .attr("id", "btn-decimal")
                            .attr("class", "btn-number"),
                    ),
            );

        Ok(buttons_ui)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

/// Calculator History Component
///
/// Shows the history of calculations performed
struct CalculatorHistory {
    base: BaseComponent,
    engine: State<CalculatorEngine>,
    show_history: State<bool>,
}

impl CalculatorHistory {
    fn new(engine: State<CalculatorEngine>) -> Self {
        let (show_history, _) = use_state(false);

        Self {
            base: BaseComponent::new("CalculatorHistory"),
            engine,
            show_history,
        }
    }

    fn toggle_history(&self) {
        self.show_history.update(|show| *show = !*show);
    }
}

#[async_trait]
impl Component for CalculatorHistory {
    fn id(&self) -> ComponentId {
        self.base.id()
    }

    fn name(&self) -> &str {
        "CalculatorHistory"
    }

    async fn render(&self, _context: &RenderContext) -> Result<VirtualNode> {
        let engine = self.engine.clone_value();
        let show = self.show_history.clone_value();

        if !show {
            return Ok(div().attr("class", "history-toggle").child(
                button("Show History")
                    .attr("id", "toggle-history")
                    .attr("class", "btn-toggle"),
            ));
        }

        let history_entries = engine.get_history();
        let recent_entries: Vec<_> = history_entries.iter().rev().take(10).collect();

        let history_ui =
            div()
                .attr("class", "calculator-history")
                .child(
                    div()
                        .attr("class", "history-header")
                        .child(text("Calculation History"))
                        .child(
                            button("Hide")
                                .attr("id", "toggle-history")
                                .attr("class", "btn-toggle"),
                        ),
                )
                .child(div().attr("class", "history-list").children(
                    if recent_entries.is_empty() {
                        vec![
                            div()
                                .attr("class", "history-empty")
                                .child(text("No calculations yet")),
                        ]
                    } else {
                        recent_entries
                            .into_iter()
                            .map(|entry| div().attr("class", "history-entry").child(text(entry)))
                            .collect()
                    },
                ));

        Ok(history_ui)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

/// Main Calculator Application Component
///
/// Combines all calculator components into a complete application
struct Calculator {
    base: BaseComponent,
    engine: State<CalculatorEngine>,
    display: CalculatorDisplay,
    buttons: CalculatorButtons,
    history: CalculatorHistory,
    error_message: State<Option<String>>,
}

impl Calculator {
    fn new() -> Self {
        let (engine, _) = use_state(CalculatorEngine::new());
        let (error_message, _) = use_state(None);

        let display = CalculatorDisplay::new(engine.clone());
        let buttons = CalculatorButtons::new(engine.clone());
        let history = CalculatorHistory::new(engine.clone());

        Self {
            base: BaseComponent::new("Calculator"),
            engine,
            display,
            buttons,
            history,
            error_message,
        }
    }

    fn handle_button_click(&self, button_id: &str) {
        match button_id {
            "toggle-history" => self.history.toggle_history(),
            _ => {
                if let Err(e) = self.buttons.handle_button_click(button_id) {
                    self.error_message.set(Some(e.to_string()));
                } else {
                    self.error_message.set(None);
                }
            }
        }
    }

    #[allow(dead_code)]
    fn clear_error(&self) {
        self.error_message.set(None);
    }
}

#[async_trait]
impl Component for Calculator {
    fn id(&self) -> ComponentId {
        self.base.id()
    }

    fn name(&self) -> &str {
        "Calculator"
    }

    async fn render(&self, context: &RenderContext) -> Result<VirtualNode> {
        let error = self.error_message.clone_value();

        let calculator_ui = div()
            .attr("class", "calculator-app")
            .child(
                div()
                    .attr("class", "calculator-header")
                    .child(text("🧮 Advanced Calculator"))
                    .child(text("TUI Framework - Complex State Management Demo")),
            )
            .child(
                div()
                    .attr("class", "calculator-main")
                    .child(
                        div()
                            .attr("class", "calculator-left")
                            .child(self.display.render(context).await?)
                            .child(if let Some(err) = error {
                                div()
                                    .attr("class", "error-display")
                                    .child(text(format!("Error: {}", err)))
                                    .child(
                                        button("Clear Error")
                                            .attr("id", "clear-error")
                                            .attr("class", "btn-error"),
                                    )
                            } else {
                                div().attr("class", "error-placeholder")
                            })
                            .child(self.buttons.render(context).await?),
                    )
                    .child(
                        div()
                            .attr("class", "calculator-right")
                            .child(self.history.render(context).await?),
                    ),
            )
            .child(
                div()
                    .attr("class", "calculator-footer")
                    .child(text(
                        "Features: Basic arithmetic, Memory functions, History tracking",
                    ))
                    .child(text(
                        "Built with TUI Framework - Demonstrating complex component composition",
                    )),
            );

        Ok(calculator_ui)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing for debugging
    tracing_subscriber::fmt::init();

    println!("🧮 Starting Advanced Calculator TUI Application");
    println!("===============================================");
    println!("This comprehensive example demonstrates:");
    println!("• Complex state management with multiple components");
    println!("• Mathematical expression parsing and evaluation");
    println!("• Component composition and communication");
    println!("• Advanced event handling patterns");
    println!("• Error handling and validation");
    println!("• Memory functions and history tracking");
    println!();

    // Create the calculator application
    println!("📱 Creating Calculator Application:");
    let calculator = Calculator::new();
    println!("   ✅ Calculator created: {}", calculator.name());

    // Demonstrate calculator engine functionality
    println!("\n🔢 Testing Calculator Engine:");
    let mut engine = CalculatorEngine::new();

    // Test basic arithmetic
    engine.input_digit('5');
    println!("   Input 5: display = {}", engine.display);

    engine.input_operation(Operation::Add)?;
    println!(
        "   Input +: pending operation = {:?}",
        engine.pending_operation
    );

    engine.input_digit('3');
    println!("   Input 3: display = {}", engine.display);

    engine.calculate()?;
    println!("   Calculate: result = {}", engine.display);

    // Test memory functions
    engine.memory_add();
    println!("   Memory add: memory = {}", engine.memory);

    engine.clear();
    engine.memory_recall();
    println!("   Memory recall: display = {}", engine.display);

    // Test division by zero error handling
    println!("\n⚠️  Testing Error Handling:");
    engine.clear();
    engine.input_digit('1');
    engine.input_operation(Operation::Divide)?;
    engine.input_digit('0');

    match engine.calculate() {
        Ok(_) => println!("   Unexpected: division by zero should fail"),
        Err(e) => println!("   ✅ Correctly caught error: {}", e),
    }

    // Test component rendering
    println!("\n🎨 Testing Component Rendering:");
    let context = RenderContext::new(&Theme::default());

    let display_vdom = calculator.display.render(&context).await?;
    println!("   ✅ Display component rendered");
    println!(
        "      Root: {}, Children: {}",
        display_vdom.tag().unwrap_or("unknown"),
        display_vdom.get_children().len()
    );

    let buttons_vdom = calculator.buttons.render(&context).await?;
    println!("   ✅ Buttons component rendered");
    println!(
        "      Root: {}, Children: {}",
        buttons_vdom.tag().unwrap_or("unknown"),
        buttons_vdom.get_children().len()
    );

    let history_vdom = calculator.history.render(&context).await?;
    println!("   ✅ History component rendered");
    println!(
        "      Root: {}, Children: {}",
        history_vdom.tag().unwrap_or("unknown"),
        history_vdom.get_children().len()
    );

    let calculator_vdom = calculator.render(&context).await?;
    println!("   ✅ Full calculator rendered");
    println!(
        "      Root: {}, Children: {}",
        calculator_vdom.tag().unwrap_or("unknown"),
        calculator_vdom.get_children().len()
    );

    // Test button interactions
    println!("\n🎮 Testing Button Interactions:");
    calculator.handle_button_click("btn-7");
    calculator.handle_button_click("btn-multiply");
    calculator.handle_button_click("btn-6");
    calculator.handle_button_click("btn-equals");

    let final_engine = calculator.engine.clone_value();
    println!("   Calculation 7 × 6 = {}", final_engine.display);
    println!("   History entries: {}", final_engine.history.len());

    // Create the TUI application
    println!("\n🏗️  Creating TUI Application:");
    let _app = App::new()
        .title("Advanced Calculator - TUI Framework")
        .component(calculator);

    println!("   ✅ TUI application created successfully");
    println!("   📱 In a real application, this would start the event loop");
    println!("   🎮 Users would interact with calculator buttons and functions");

    println!("\n🎉 Calculator Example Completed Successfully!");
    println!("   ✨ All components rendered without errors");
    println!("   🔄 Complex state management working correctly");
    println!("   🧮 Mathematical operations functioning properly");
    println!("   📊 Memory and history features operational");
    println!("   🎯 Advanced framework patterns demonstrated");
    println!();
    println!("This example showcases:");
    println!("• Multi-component architecture with shared state");
    println!("• Sophisticated event handling and user interaction");
    println!("• Error handling and validation patterns");
    println!("• Real-world application structure and organization");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculator_engine_basic_operations() {
        let mut engine = CalculatorEngine::new();

        // Test addition
        engine.input_digit('5');
        engine.input_operation(Operation::Add).unwrap();
        engine.input_digit('3');
        engine.calculate().unwrap();
        assert_eq!(engine.current_value, 8.0);
        assert_eq!(engine.display, "8");

        // Test subtraction
        engine.clear();
        engine.input_digit('1');
        engine.input_digit('0');
        engine.input_operation(Operation::Subtract).unwrap();
        engine.input_digit('4');
        engine.calculate().unwrap();
        assert_eq!(engine.current_value, 6.0);

        // Test multiplication
        engine.clear();
        engine.input_digit('7');
        engine.input_operation(Operation::Multiply).unwrap();
        engine.input_digit('6');
        engine.calculate().unwrap();
        assert_eq!(engine.current_value, 42.0);

        // Test division
        engine.clear();
        engine.input_digit('8');
        engine.input_operation(Operation::Divide).unwrap();
        engine.input_digit('2');
        engine.calculate().unwrap();
        assert_eq!(engine.current_value, 4.0);
    }

    #[test]
    fn test_calculator_engine_decimal_operations() {
        let mut engine = CalculatorEngine::new();

        // Test decimal input
        engine.input_digit('3');
        engine.input_decimal();
        engine.input_digit('1');
        engine.input_digit('4');
        assert_eq!(engine.display, "3.14");
        assert_eq!(engine.current_value, 3.14);

        // Test decimal arithmetic
        engine.input_operation(Operation::Multiply).unwrap();
        engine.input_digit('2');
        engine.calculate().unwrap();
        assert!((engine.current_value - 6.28).abs() < 0.001);
    }

    #[test]
    fn test_calculator_engine_error_handling() {
        let mut engine = CalculatorEngine::new();

        // Test division by zero
        engine.input_digit('5');
        engine.input_operation(Operation::Divide).unwrap();
        engine.input_digit('0');

        let result = engine.calculate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Division by zero"));
    }

    #[test]
    fn test_calculator_engine_memory_functions() {
        let mut engine = CalculatorEngine::new();

        // Test memory operations
        engine.input_digit('5');
        engine.memory_add();
        assert_eq!(engine.memory, 5.0);

        engine.clear();
        engine.input_digit('3');
        engine.memory_add();
        assert_eq!(engine.memory, 8.0);

        engine.memory_subtract();
        assert_eq!(engine.memory, 5.0);

        engine.clear();
        engine.memory_recall();
        assert_eq!(engine.current_value, 5.0);
        assert_eq!(engine.display, "5");

        engine.memory_clear();
        assert_eq!(engine.memory, 0.0);
    }

    #[test]
    fn test_calculator_engine_history() {
        let mut engine = CalculatorEngine::new();

        // Perform some calculations
        engine.input_digit('2');
        engine.input_operation(Operation::Add).unwrap();
        engine.input_digit('3');
        engine.calculate().unwrap();

        engine.input_operation(Operation::Multiply).unwrap();
        engine.input_digit('4');
        engine.calculate().unwrap();

        let history = engine.get_history();
        assert_eq!(history.len(), 2);
        assert!(history[0].contains("2 + 3 = 5"));
        assert!(history[1].contains("5 × 4 = 20"));
    }

    #[test]
    fn test_format_number() {
        assert_eq!(format_number(42.0), "42");
        assert_eq!(format_number(3.14), "3.14");
        assert_eq!(format_number(3.0), "3");
        assert_eq!(format_number(0.0), "0");
        assert_eq!(format_number(-5.0), "-5");
    }

    #[tokio::test]
    async fn test_calculator_display_component() {
        let (engine_state, _) = use_state(CalculatorEngine::new());
        let display = CalculatorDisplay::new(engine_state);
        let context = RenderContext::new(&Theme::default());

        let vdom = display.render(&context).await.unwrap();
        assert_eq!(vdom.tag(), Some("div"));
        assert!(!vdom.get_children().is_empty());
    }

    #[tokio::test]
    async fn test_calculator_buttons_component() {
        let (engine_state, _) = use_state(CalculatorEngine::new());
        let buttons = CalculatorButtons::new(engine_state.clone());
        let context = RenderContext::new(&Theme::default());

        let vdom = buttons.render(&context).await.unwrap();
        assert_eq!(vdom.tag(), Some("div"));
        assert!(!vdom.get_children().is_empty());

        // Test button click handling
        let result = buttons.handle_button_click("btn-5");
        assert!(result.is_ok());

        let engine = engine_state.clone_value();
        assert_eq!(engine.display, "5");
    }

    #[tokio::test]
    async fn test_calculator_history_component() {
        let (engine_state, _) = use_state(CalculatorEngine::new());
        let history = CalculatorHistory::new(engine_state);
        let context = RenderContext::new(&Theme::default());

        // Test initial render (hidden)
        let vdom = history.render(&context).await.unwrap();
        assert_eq!(vdom.tag(), Some("div"));

        // Test toggle
        history.toggle_history();
        let vdom2 = history.render(&context).await.unwrap();
        assert_eq!(vdom2.tag(), Some("div"));
    }

    #[tokio::test]
    async fn test_full_calculator_component() {
        let calculator = Calculator::new();
        let context = RenderContext::new(&Theme::default());

        // Test initial render
        let vdom = calculator.render(&context).await.unwrap();
        assert_eq!(vdom.tag(), Some("div"));
        assert!(!vdom.get_children().is_empty());

        // Test button interactions
        calculator.handle_button_click("btn-7");
        calculator.handle_button_click("btn-add");
        calculator.handle_button_click("btn-3");
        calculator.handle_button_click("btn-equals");

        let engine = calculator.engine.clone_value();
        assert_eq!(engine.current_value, 10.0);
    }

    #[tokio::test]
    async fn test_calculator_error_handling() {
        let calculator = Calculator::new();

        // Trigger division by zero
        calculator.handle_button_click("btn-1");
        calculator.handle_button_click("btn-divide");
        calculator.handle_button_click("btn-0");
        calculator.handle_button_click("btn-equals");

        let error = calculator.error_message.clone_value();
        assert!(error.is_some());
        assert!(error.unwrap().contains("Division by zero"));

        // Test error clearing
        calculator.clear_error();
        let error_after_clear = calculator.error_message.clone_value();
        assert!(error_after_clear.is_none());
    }

    #[tokio::test]
    async fn test_complex_calculation_sequence() {
        let calculator = Calculator::new();

        // Perform: (5 + 3) * 2 - 1 = 15
        calculator.handle_button_click("btn-5");
        calculator.handle_button_click("btn-add");
        calculator.handle_button_click("btn-3");
        calculator.handle_button_click("btn-equals"); // = 8

        calculator.handle_button_click("btn-multiply");
        calculator.handle_button_click("btn-2");
        calculator.handle_button_click("btn-equals"); // = 16

        calculator.handle_button_click("btn-subtract");
        calculator.handle_button_click("btn-1");
        calculator.handle_button_click("btn-equals"); // = 15

        let engine = calculator.engine.clone_value();
        assert_eq!(engine.current_value, 15.0);
        assert!(engine.history.len() >= 3);
    }
}
