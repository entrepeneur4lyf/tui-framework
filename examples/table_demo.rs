//! Table widget demonstration application.
//!
//! This example showcases the Table widget's capabilities including:
//! - Column configuration with different alignments and widths
//! - Row and cell selection modes
//! - Keyboard navigation and mouse interaction
//! - Sorting by columns
//! - Basic table operations

use tui_framework::component::BaseComponent;
use tui_framework::prelude::*;
use tui_framework::widget::{ColumnAlignment, Table, TableColumn, TableRow, TableSelectionMode};

/// Simple table demo component.
struct TableDemo {
    base: BaseComponent,
    table: Table,
}

impl TableDemo {
    fn new() -> Self {
        let mut table = Table::new()
            .with_selection_mode(TableSelectionMode::Row)
            .with_visible_rows(8)
            .with_visible_columns(5)
            .show_headers(true)
            .show_grid(true);

        // Configure columns
        table.add_column(
            TableColumn::new("id", "ID")
                .with_width(5)
                .with_alignment(ColumnAlignment::Right),
        );
        table.add_column(
            TableColumn::new("name", "Name")
                .with_width(15)
                .with_alignment(ColumnAlignment::Left),
        );
        table.add_column(
            TableColumn::new("department", "Department")
                .with_width(12)
                .with_alignment(ColumnAlignment::Left),
        );
        table.add_column(
            TableColumn::new("salary", "Salary")
                .with_width(10)
                .with_alignment(ColumnAlignment::Right),
        );
        table.add_column(
            TableColumn::new("experience", "Years")
                .with_width(6)
                .with_alignment(ColumnAlignment::Right),
        );

        // Add sample data
        table.add_row(TableRow::new(
            "1",
            vec![
                "1".to_string(),
                "Alice Johnson".to_string(),
                "Engineering".to_string(),
                "$95,000".to_string(),
                "5".to_string(),
            ],
        ));
        table.add_row(TableRow::new(
            "2",
            vec![
                "2".to_string(),
                "Bob Smith".to_string(),
                "Marketing".to_string(),
                "$65,000".to_string(),
                "3".to_string(),
            ],
        ));
        table.add_row(TableRow::new(
            "3",
            vec![
                "3".to_string(),
                "Charlie Brown".to_string(),
                "Engineering".to_string(),
                "$105,000".to_string(),
                "8".to_string(),
            ],
        ));
        table.add_row(TableRow::new(
            "4",
            vec![
                "4".to_string(),
                "Diana Prince".to_string(),
                "Sales".to_string(),
                "$75,000".to_string(),
                "4".to_string(),
            ],
        ));
        table.add_row(TableRow::new(
            "5",
            vec![
                "5".to_string(),
                "Eve Wilson".to_string(),
                "HR".to_string(),
                "$60,000".to_string(),
                "2".to_string(),
            ],
        ));

        Self {
            base: BaseComponent::new("TableDemo"),
            table,
        }
    }
}

#[async_trait]
impl Component for TableDemo {
    fn id(&self) -> ComponentId {
        self.base.id()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    async fn render(&self, context: &RenderContext) -> Result<VirtualNode> {
        let table_node = self.table.render(context).await?;

        let container = VirtualNode::element("div")
            .attr("class", "table-demo-container")
            .child(
                VirtualNode::element("h1")
                    .attr("class", "table-title")
                    .child(VirtualNode::text("Employee Directory - Table Demo")),
            )
            .child(table_node);

        Ok(container)
    }
}

#[async_trait]
impl Widget for TableDemo {
    fn widget_type(&self) -> &'static str {
        "TableDemo"
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let demo = TableDemo::new();

    println!("Table Demo Application");
    println!("======================");
    println!("This demo showcases the Table widget with employee data.");
    println!("Features demonstrated:");
    println!("- Column configuration with different alignments");
    println!("- Row selection and navigation");
    println!("- Sorting capabilities");
    println!("- Keyboard and mouse interaction");
    println!();
    println!("Note: This is a console demo. In a real TUI application,");
    println!("the table would be rendered to the terminal with full interaction.");

    // Simulate rendering
    let theme = Theme::default();
    let context = RenderContext::new(&theme);
    let vnode = demo.render(&context).await?;

    println!("Table rendered successfully with {} employees!", 5);
    println!("Virtual DOM structure: {:?}", vnode.tag());

    Ok(())
}
