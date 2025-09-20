//! Table widget implementation for displaying tabular data with columns and rows.

use crate::component::{BaseComponent, Component, ComponentId};
use crate::error::Result;
use crate::event::types::{KeyEvent, MouseButton, MouseEvent, MouseEventType, NcKey};
use crate::render::{RenderContext, VirtualNode};
use crate::style::properties::Style;
use crate::widget::Widget;
use async_trait::async_trait;
use std::sync::Arc;

/// Column alignment options for table cells.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColumnAlignment {
    /// Left-aligned text
    Left,
    /// Center-aligned text
    Center,
    /// Right-aligned text
    Right,
}

/// Sort direction for table columns.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortDirection {
    /// Ascending sort
    Ascending,
    /// Descending sort
    Descending,
}

/// Table column definition.
#[derive(Debug, Clone)]
pub struct TableColumn {
    /// Column identifier
    pub id: String,
    /// Column header text
    pub title: String,
    /// Column width (in characters, None for auto-sizing)
    pub width: Option<usize>,
    /// Text alignment for this column
    pub alignment: ColumnAlignment,
    /// Whether this column is sortable
    pub sortable: bool,
    /// Whether this column is resizable
    pub resizable: bool,
}

impl TableColumn {
    /// Create a new table column.
    pub fn new(id: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            width: None,
            alignment: ColumnAlignment::Left,
            sortable: true,
            resizable: true,
        }
    }

    /// Set the column width.
    pub fn with_width(mut self, width: usize) -> Self {
        self.width = Some(width);
        self
    }

    /// Set the column alignment.
    pub fn with_alignment(mut self, alignment: ColumnAlignment) -> Self {
        self.alignment = alignment;
        self
    }

    /// Set whether the column is sortable.
    pub fn sortable(mut self, sortable: bool) -> Self {
        self.sortable = sortable;
        self
    }

    /// Set whether the column is resizable.
    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }
}

/// Table row data structure.
#[derive(Debug, Clone)]
pub struct TableRow {
    /// Unique identifier for the row
    pub id: String,
    /// Cell data for each column (indexed by column order)
    pub cells: Vec<String>,
    /// Whether the row is enabled/selectable
    pub enabled: bool,
    /// Custom data associated with the row
    pub data: Option<String>,
}

impl TableRow {
    /// Create a new table row.
    pub fn new(id: impl Into<String>, cells: Vec<String>) -> Self {
        Self {
            id: id.into(),
            cells,
            enabled: true,
            data: None,
        }
    }

    /// Create a new table row with custom data.
    pub fn with_data(id: impl Into<String>, cells: Vec<String>, data: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            cells,
            enabled: true,
            data: Some(data.into()),
        }
    }

    /// Set whether the row is enabled.
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Get cell content for a specific column index.
    pub fn get_cell(&self, column_index: usize) -> Option<&String> {
        self.cells.get(column_index)
    }

    /// Set cell content for a specific column index.
    pub fn set_cell(&mut self, column_index: usize, content: String) {
        if column_index < self.cells.len() {
            self.cells[column_index] = content;
        }
    }
}

/// Selection mode for the table widget.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TableSelectionMode {
    /// No selection allowed
    None,
    /// Single row selection
    Row,
    /// Single cell selection
    Cell,
    /// Multiple row selection
    MultipleRows,
    /// Multiple cell selection
    MultipleCells,
}

/// Table cell position.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CellPosition {
    /// Row index
    pub row: usize,
    /// Column index
    pub column: usize,
}

impl CellPosition {
    /// Create a new cell position.
    pub fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }
}

/// Sort configuration for a table.
#[derive(Debug, Clone)]
pub struct TableSort {
    /// Column index to sort by
    pub column_index: usize,
    /// Sort direction
    pub direction: SortDirection,
}

impl TableSort {
    /// Create a new table sort configuration.
    pub fn new(column_index: usize, direction: SortDirection) -> Self {
        Self {
            column_index,
            direction,
        }
    }
}

/// A table widget for displaying tabular data with columns, rows, sorting, and selection.
pub struct Table {
    base: BaseComponent,
    columns: Vec<TableColumn>,
    rows: Vec<TableRow>,
    selection_mode: TableSelectionMode,
    selected_rows: Vec<usize>,
    selected_cells: Vec<CellPosition>,
    focused_cell: Option<CellPosition>,
    scroll_offset_row: usize,
    scroll_offset_col: usize,
    visible_rows: usize,
    visible_columns: usize,
    sort: Option<TableSort>,
    style: Style,
    show_headers: bool,
    show_grid: bool,
    on_selection_changed: Option<Arc<dyn Fn(&[usize], &[CellPosition]) + Send + Sync>>,
    on_cell_activated: Option<Arc<dyn Fn(CellPosition, &str) + Send + Sync>>,
    on_sort_changed: Option<Arc<dyn Fn(usize, SortDirection) + Send + Sync>>,
}

impl Table {
    /// Create a new table widget.
    pub fn new() -> Self {
        Self {
            base: BaseComponent::new("Table"),
            columns: Vec::new(),
            rows: Vec::new(),
            selection_mode: TableSelectionMode::Row,
            selected_rows: Vec::new(),
            selected_cells: Vec::new(),
            focused_cell: None,
            scroll_offset_row: 0,
            scroll_offset_col: 0,
            visible_rows: 10,
            visible_columns: 5,
            sort: None,
            style: Style::default(),
            show_headers: true,
            show_grid: true,
            on_selection_changed: None,
            on_cell_activated: None,
            on_sort_changed: None,
        }
    }

    /// Set the table columns.
    pub fn with_columns(mut self, columns: Vec<TableColumn>) -> Self {
        self.columns = columns;
        self
    }

    /// Add a column to the table.
    pub fn add_column(&mut self, column: TableColumn) {
        self.columns.push(column);
    }

    /// Set the table rows.
    pub fn with_rows(mut self, rows: Vec<TableRow>) -> Self {
        self.rows = rows;
        self
    }

    /// Add a row to the table.
    pub fn add_row(&mut self, row: TableRow) {
        self.rows.push(row);
    }

    /// Set the selection mode.
    pub fn with_selection_mode(mut self, mode: TableSelectionMode) -> Self {
        self.selection_mode = mode;
        self
    }

    /// Set the number of visible rows.
    pub fn with_visible_rows(mut self, count: usize) -> Self {
        self.visible_rows = count.max(1);
        self
    }

    /// Set the number of visible columns.
    pub fn with_visible_columns(mut self, count: usize) -> Self {
        self.visible_columns = count.max(1);
        self
    }

    /// Set whether to show column headers.
    pub fn show_headers(mut self, show: bool) -> Self {
        self.show_headers = show;
        self
    }

    /// Set whether to show grid lines.
    pub fn show_grid(mut self, show: bool) -> Self {
        self.show_grid = show;
        self
    }

    /// Set the table style.
    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Set the selection changed callback.
    pub fn on_selection_changed<F>(mut self, handler: F) -> Self
    where
        F: Fn(&[usize], &[CellPosition]) + Send + Sync + 'static,
    {
        self.on_selection_changed = Some(Arc::new(handler));
        self
    }

    /// Set the cell activated callback.
    pub fn on_cell_activated<F>(mut self, handler: F) -> Self
    where
        F: Fn(CellPosition, &str) + Send + Sync + 'static,
    {
        self.on_cell_activated = Some(Arc::new(handler));
        self
    }

    /// Set the sort changed callback.
    pub fn on_sort_changed<F>(mut self, handler: F) -> Self
    where
        F: Fn(usize, SortDirection) + Send + Sync + 'static,
    {
        self.on_sort_changed = Some(Arc::new(handler));
        self
    }

    /// Get the currently selected rows.
    pub fn selected_rows(&self) -> &[usize] {
        &self.selected_rows
    }

    /// Get the currently selected cells.
    pub fn selected_cells(&self) -> &[CellPosition] {
        &self.selected_cells
    }

    /// Get the focused cell position.
    pub fn focused_cell(&self) -> Option<CellPosition> {
        self.focused_cell
    }

    /// Get the number of rows.
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    /// Get the number of columns.
    pub fn column_count(&self) -> usize {
        self.columns.len()
    }

    /// Get a row by index.
    pub fn get_row(&self, index: usize) -> Option<&TableRow> {
        self.rows.get(index)
    }

    /// Get a mutable row by index.
    pub fn get_row_mut(&mut self, index: usize) -> Option<&mut TableRow> {
        self.rows.get_mut(index)
    }

    /// Get a column by index.
    pub fn get_column(&self, index: usize) -> Option<&TableColumn> {
        self.columns.get(index)
    }

    /// Get cell content at the specified position.
    pub fn get_cell_content(&self, position: CellPosition) -> Option<&String> {
        self.rows.get(position.row)?.get_cell(position.column)
    }

    /// Set cell content at the specified position.
    pub fn set_cell_content(&mut self, position: CellPosition, content: String) {
        if let Some(row) = self.rows.get_mut(position.row) {
            row.set_cell(position.column, content);
        }
    }

    /// Clear all selections.
    pub fn clear_selection(&mut self) {
        self.selected_rows.clear();
        self.selected_cells.clear();
        self.focused_cell = None;
    }

    /// Select a row by index.
    pub fn select_row(&mut self, row_index: usize) {
        if row_index >= self.rows.len() {
            return;
        }

        match self.selection_mode {
            TableSelectionMode::None => return,
            TableSelectionMode::Row => {
                self.selected_rows.clear();
                self.selected_rows.push(row_index);
                self.focused_cell = Some(CellPosition::new(row_index, 0));
            }
            TableSelectionMode::MultipleRows => {
                if !self.selected_rows.contains(&row_index) {
                    self.selected_rows.push(row_index);
                }
                self.focused_cell = Some(CellPosition::new(row_index, 0));
            }
            _ => {
                // For cell selection modes, select the first cell of the row
                self.select_cell(CellPosition::new(row_index, 0));
            }
        }

        if let Some(ref callback) = self.on_selection_changed {
            callback(&self.selected_rows, &self.selected_cells);
        }
    }

    /// Select a cell by position.
    pub fn select_cell(&mut self, position: CellPosition) {
        if position.row >= self.rows.len() || position.column >= self.columns.len() {
            return;
        }

        match self.selection_mode {
            TableSelectionMode::None => return,
            TableSelectionMode::Cell => {
                self.selected_cells.clear();
                self.selected_cells.push(position);
                self.focused_cell = Some(position);
            }
            TableSelectionMode::MultipleCells => {
                if !self.selected_cells.contains(&position) {
                    self.selected_cells.push(position);
                }
                self.focused_cell = Some(position);
            }
            TableSelectionMode::Row | TableSelectionMode::MultipleRows => {
                // For row selection modes, select the entire row
                self.select_row(position.row);
                return;
            }
        }

        if let Some(ref callback) = self.on_selection_changed {
            callback(&self.selected_rows, &self.selected_cells);
        }
    }

    /// Toggle row selection.
    pub fn toggle_row_selection(&mut self, row_index: usize) {
        if row_index >= self.rows.len() {
            return;
        }

        match self.selection_mode {
            TableSelectionMode::MultipleRows => {
                if let Some(pos) = self.selected_rows.iter().position(|&r| r == row_index) {
                    self.selected_rows.remove(pos);
                } else {
                    self.selected_rows.push(row_index);
                }
                self.focused_cell = Some(CellPosition::new(row_index, 0));
            }
            _ => {
                self.select_row(row_index);
            }
        }

        if let Some(ref callback) = self.on_selection_changed {
            callback(&self.selected_rows, &self.selected_cells);
        }
    }

    /// Sort the table by a column.
    pub fn sort_by_column(&mut self, column_index: usize, direction: SortDirection) {
        if column_index >= self.columns.len() {
            return;
        }

        let column = &self.columns[column_index];
        if !column.sortable {
            return;
        }

        // Store the sort configuration
        self.sort = Some(TableSort::new(column_index, direction));

        // Sort the rows
        self.rows.sort_by(|a, b| {
            let empty_string = String::new();
            let cell_a = a.get_cell(column_index).unwrap_or(&empty_string);
            let cell_b = b.get_cell(column_index).unwrap_or(&empty_string);

            let comparison = cell_a.cmp(cell_b);
            match direction {
                SortDirection::Ascending => comparison,
                SortDirection::Descending => comparison.reverse(),
            }
        });

        // Clear selections as row indices have changed
        self.clear_selection();

        if let Some(ref callback) = self.on_sort_changed {
            callback(column_index, direction);
        }
    }

    /// Get the current sort configuration.
    pub fn current_sort(&self) -> Option<&TableSort> {
        self.sort.as_ref()
    }

    /// Move focus to the next cell (right, wrapping to next row).
    pub fn move_focus_next(&mut self) {
        if let Some(current) = self.focused_cell {
            let mut new_pos = current;
            new_pos.column += 1;

            if new_pos.column >= self.columns.len() {
                new_pos.column = 0;
                new_pos.row += 1;
                if new_pos.row >= self.rows.len() {
                    new_pos.row = 0;
                }
            }

            self.focused_cell = Some(new_pos);
            self.ensure_cell_visible(new_pos);
        } else if !self.rows.is_empty() && !self.columns.is_empty() {
            self.focused_cell = Some(CellPosition::new(0, 0));
        }
    }

    /// Move focus to the previous cell (left, wrapping to previous row).
    pub fn move_focus_previous(&mut self) {
        if let Some(current) = self.focused_cell {
            let mut new_pos = current;

            if new_pos.column == 0 {
                if new_pos.row == 0 {
                    new_pos.row = self.rows.len().saturating_sub(1);
                } else {
                    new_pos.row -= 1;
                }
                new_pos.column = self.columns.len().saturating_sub(1);
            } else {
                new_pos.column -= 1;
            }

            self.focused_cell = Some(new_pos);
            self.ensure_cell_visible(new_pos);
        } else if !self.rows.is_empty() && !self.columns.is_empty() {
            let last_row = self.rows.len() - 1;
            let last_col = self.columns.len() - 1;
            self.focused_cell = Some(CellPosition::new(last_row, last_col));
        }
    }

    /// Move focus up one row.
    pub fn move_focus_up(&mut self) {
        if let Some(current) = self.focused_cell {
            if current.row > 0 {
                let new_pos = CellPosition::new(current.row - 1, current.column);
                self.focused_cell = Some(new_pos);
                self.ensure_cell_visible(new_pos);
            }
        }
    }

    /// Move focus down one row.
    pub fn move_focus_down(&mut self) {
        if let Some(current) = self.focused_cell {
            if current.row + 1 < self.rows.len() {
                let new_pos = CellPosition::new(current.row + 1, current.column);
                self.focused_cell = Some(new_pos);
                self.ensure_cell_visible(new_pos);
            }
        }
    }

    /// Move focus left one column.
    pub fn move_focus_left(&mut self) {
        if let Some(current) = self.focused_cell {
            if current.column > 0 {
                let new_pos = CellPosition::new(current.row, current.column - 1);
                self.focused_cell = Some(new_pos);
                self.ensure_cell_visible(new_pos);
            }
        }
    }

    /// Move focus right one column.
    pub fn move_focus_right(&mut self) {
        if let Some(current) = self.focused_cell {
            if current.column + 1 < self.columns.len() {
                let new_pos = CellPosition::new(current.row, current.column + 1);
                self.focused_cell = Some(new_pos);
                self.ensure_cell_visible(new_pos);
            }
        }
    }

    /// Move focus to the first cell.
    pub fn move_focus_home(&mut self) {
        if !self.rows.is_empty() && !self.columns.is_empty() {
            let new_pos = CellPosition::new(0, 0);
            self.focused_cell = Some(new_pos);
            self.scroll_offset_row = 0;
            self.scroll_offset_col = 0;
        }
    }

    /// Move focus to the last cell.
    pub fn move_focus_end(&mut self) {
        if !self.rows.is_empty() && !self.columns.is_empty() {
            let last_row = self.rows.len() - 1;
            let last_col = self.columns.len() - 1;
            let new_pos = CellPosition::new(last_row, last_col);
            self.focused_cell = Some(new_pos);
            self.ensure_cell_visible(new_pos);
        }
    }

    /// Move focus up one page.
    pub fn move_focus_page_up(&mut self) {
        if let Some(current) = self.focused_cell {
            let new_row = current.row.saturating_sub(self.visible_rows);
            let new_pos = CellPosition::new(new_row, current.column);
            self.focused_cell = Some(new_pos);
            self.ensure_cell_visible(new_pos);
        }
    }

    /// Move focus down one page.
    pub fn move_focus_page_down(&mut self) {
        if let Some(current) = self.focused_cell {
            let new_row = (current.row + self.visible_rows).min(self.rows.len().saturating_sub(1));
            let new_pos = CellPosition::new(new_row, current.column);
            self.focused_cell = Some(new_pos);
            self.ensure_cell_visible(new_pos);
        }
    }

    /// Ensure a cell is visible by adjusting scroll offsets.
    fn ensure_cell_visible(&mut self, position: CellPosition) {
        // Adjust row scrolling
        if position.row < self.scroll_offset_row {
            self.scroll_offset_row = position.row;
        } else if position.row >= self.scroll_offset_row + self.visible_rows {
            self.scroll_offset_row = position.row.saturating_sub(self.visible_rows - 1);
        }

        // Adjust column scrolling
        if position.column < self.scroll_offset_col {
            self.scroll_offset_col = position.column;
        } else if position.column >= self.scroll_offset_col + self.visible_columns {
            self.scroll_offset_col = position.column.saturating_sub(self.visible_columns - 1);
        }
    }

    /// Activate the currently focused cell.
    pub fn activate_focused_cell(&self) {
        if let Some(position) = self.focused_cell {
            if let Some(content) = self.get_cell_content(position) {
                if let Some(ref callback) = self.on_cell_activated {
                    callback(position, content);
                }
            }
        }
    }

    /// Handle keyboard input for table navigation and interaction.
    pub fn handle_key_event(&mut self, event: &KeyEvent) -> bool {
        match event.key {
            NcKey::Up => {
                self.move_focus_up();
                true
            }
            NcKey::Down => {
                self.move_focus_down();
                true
            }
            NcKey::Left => {
                self.move_focus_left();
                true
            }
            NcKey::Right => {
                self.move_focus_right();
                true
            }
            NcKey::Home => {
                self.move_focus_home();
                true
            }
            NcKey::End => {
                self.move_focus_end();
                true
            }
            NcKey::PgUp => {
                self.move_focus_page_up();
                true
            }
            NcKey::PgDown => {
                self.move_focus_page_down();
                true
            }
            NcKey::Tab => {
                self.move_focus_next();
                true
            }
            NcKey::Enter => {
                if let Some(position) = self.focused_cell {
                    match self.selection_mode {
                        TableSelectionMode::None => {}
                        TableSelectionMode::Cell | TableSelectionMode::MultipleCells => {
                            self.select_cell(position);
                        }
                        TableSelectionMode::Row | TableSelectionMode::MultipleRows => {
                            self.select_row(position.row);
                        }
                    }
                    self.activate_focused_cell();
                }
                true
            }
            NcKey::Space => {
                if let Some(position) = self.focused_cell {
                    match self.selection_mode {
                        TableSelectionMode::MultipleRows => {
                            self.toggle_row_selection(position.row);
                        }
                        TableSelectionMode::MultipleCells => {
                            if self.selected_cells.contains(&position) {
                                self.selected_cells.retain(|&p| p != position);
                            } else {
                                self.selected_cells.push(position);
                            }
                            if let Some(ref callback) = self.on_selection_changed {
                                callback(&self.selected_rows, &self.selected_cells);
                            }
                        }
                        _ => {
                            // For single selection modes, space acts like enter
                            match self.selection_mode {
                                TableSelectionMode::Cell => self.select_cell(position),
                                TableSelectionMode::Row => self.select_row(position.row),
                                _ => {}
                            }
                        }
                    }
                }
                true
            }
            _ => false,
        }
    }

    /// Handle mouse events for table interaction.
    pub fn handle_mouse_event(&mut self, event: &MouseEvent) -> bool {
        match event.event_type {
            MouseEventType::Press => {
                match event.button {
                    MouseButton::Left => {
                        // Calculate which cell was clicked
                        if let Some(position) =
                            self.get_cell_at_position(event.x as usize, event.y as usize)
                        {
                            self.focused_cell = Some(position);

                            match self.selection_mode {
                                TableSelectionMode::None => {}
                                TableSelectionMode::Cell | TableSelectionMode::MultipleCells => {
                                    self.select_cell(position);
                                }
                                TableSelectionMode::Row | TableSelectionMode::MultipleRows => {
                                    self.select_row(position.row);
                                }
                            }
                            return true;
                        }
                    }
                    MouseButton::Right => {
                        // Right-click for context menu (future implementation)
                        return true;
                    }
                    _ => {}
                }
            }
            MouseEventType::Scroll => {
                // Scroll the table (simplified - no delta parameter in current implementation)
                // For now, just scroll down
                let max_scroll = self.rows.len().saturating_sub(self.visible_rows);
                self.scroll_offset_row = (self.scroll_offset_row + 1).min(max_scroll);
                return true;
            }
            _ => {}
        }
        false
    }

    /// Get the cell position at the given screen coordinates.
    /// This is a simplified implementation - in a real implementation,
    /// you would need to account for column widths, headers, borders, etc.
    fn get_cell_at_position(&self, x: usize, y: usize) -> Option<CellPosition> {
        // This is a placeholder implementation
        // In a real implementation, you would:
        // 1. Account for table position and borders
        // 2. Calculate column widths and positions
        // 3. Account for header row if shown
        // 4. Map screen coordinates to cell position

        let header_offset = if self.show_headers { 1 } else { 0 };

        if y < header_offset {
            return None; // Clicked on header
        }

        let row = (y - header_offset) + self.scroll_offset_row;
        let col = (x / 10) + self.scroll_offset_col; // Assume 10 chars per column for now

        if row < self.rows.len() && col < self.columns.len() {
            Some(CellPosition::new(row, col))
        } else {
            None
        }
    }
}

impl Default for Table {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Component for Table {
    fn id(&self) -> ComponentId {
        self.base.id()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    async fn render(&self, _context: &RenderContext) -> Result<VirtualNode> {
        // Create the table virtual node
        let mut table_node = VirtualNode::element("table");

        // Add header row if enabled
        if self.show_headers && !self.columns.is_empty() {
            let mut header_row = VirtualNode::element("tr").attr("class", "table-header");

            for (col_index, column) in self.columns.iter().enumerate() {
                let header_cell = VirtualNode::element("th").attr("class", "table-header-cell");

                // Add sort indicator if this column is sorted
                let mut header_text = column.title.clone();
                if let Some(sort) = &self.sort {
                    if sort.column_index == col_index {
                        match sort.direction {
                            SortDirection::Ascending => header_text.push_str(" ↑"),
                            SortDirection::Descending => header_text.push_str(" ↓"),
                        }
                    }
                }

                let header_cell_with_text = header_cell.child(VirtualNode::text(header_text));
                header_row = header_row.child(header_cell_with_text);
            }

            table_node = table_node.child(header_row);
        }

        // Add visible data rows
        let start_row = self.scroll_offset_row;
        let end_row = (start_row + self.visible_rows).min(self.rows.len());

        for row_index in start_row..end_row {
            if let Some(row) = self.rows.get(row_index) {
                let mut row_classes = "table-row".to_string();

                // Add selection styling
                if self.selected_rows.contains(&row_index) {
                    row_classes.push_str(" table-row-selected");
                }

                // Add focus styling
                if let Some(focused) = self.focused_cell {
                    if focused.row == row_index {
                        row_classes.push_str(" table-row-focused");
                    }
                }

                let mut row_node = VirtualNode::element("tr").attr("class", row_classes);

                // Add cells for visible columns
                let start_col = self.scroll_offset_col;
                let end_col = (start_col + self.visible_columns).min(self.columns.len());

                for col_index in start_col..end_col {
                    let mut cell_classes = "table-cell".to_string();

                    // Add cell selection styling
                    let cell_pos = CellPosition::new(row_index, col_index);
                    if self.selected_cells.contains(&cell_pos) {
                        cell_classes.push_str(" table-cell-selected");
                    }

                    // Add focus styling
                    if Some(cell_pos) == self.focused_cell {
                        cell_classes.push_str(" table-cell-focused");
                    }

                    // Add cell content
                    let content = row.get_cell(col_index).unwrap_or(&String::new()).clone();
                    let cell_node = VirtualNode::element("td")
                        .attr("class", cell_classes)
                        .child(VirtualNode::text(content));

                    row_node = row_node.child(cell_node);
                }

                table_node = table_node.child(row_node);
            }
        }

        Ok(table_node)
    }
}

#[async_trait]
impl Widget for Table {
    fn widget_type(&self) -> &'static str {
        "Table"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::RenderContext;

    fn create_test_table() -> Table {
        let mut table = Table::new();

        // Add columns
        table.add_column(TableColumn::new("id", "ID").with_width(5));
        table.add_column(TableColumn::new("name", "Name").with_width(20));
        table.add_column(
            TableColumn::new("age", "Age")
                .with_width(5)
                .with_alignment(ColumnAlignment::Right),
        );

        // Add rows
        table.add_row(TableRow::new(
            "1",
            vec!["1".to_string(), "Alice".to_string(), "25".to_string()],
        ));
        table.add_row(TableRow::new(
            "2",
            vec!["2".to_string(), "Bob".to_string(), "30".to_string()],
        ));
        table.add_row(TableRow::new(
            "3",
            vec!["3".to_string(), "Charlie".to_string(), "35".to_string()],
        ));

        table
    }

    #[test]
    fn test_table_creation() {
        let table = Table::new();
        assert_eq!(table.row_count(), 0);
        assert_eq!(table.column_count(), 0);
        assert_eq!(table.selection_mode, TableSelectionMode::Row);
        assert!(table.show_headers);
        assert!(table.show_grid);
    }

    #[test]
    fn test_table_with_data() {
        let table = create_test_table();
        assert_eq!(table.row_count(), 3);
        assert_eq!(table.column_count(), 3);

        // Test column access
        let first_column = table.get_column(0).unwrap();
        assert_eq!(first_column.id, "id");
        assert_eq!(first_column.title, "ID");
        assert_eq!(first_column.width, Some(5));

        // Test row access
        let first_row = table.get_row(0).unwrap();
        assert_eq!(first_row.id, "1");
        assert_eq!(first_row.get_cell(1).unwrap(), "Alice");
    }

    #[test]
    fn test_table_selection() {
        let mut table = create_test_table();

        // Test row selection
        table.select_row(1);
        assert_eq!(table.selected_rows(), &[1]);
        assert_eq!(table.focused_cell(), Some(CellPosition::new(1, 0)));

        // Test cell selection
        table = table.with_selection_mode(TableSelectionMode::Cell);
        table.select_cell(CellPosition::new(2, 1));
        assert_eq!(table.selected_cells(), &[CellPosition::new(2, 1)]);
        assert_eq!(table.focused_cell(), Some(CellPosition::new(2, 1)));
    }

    #[test]
    fn test_table_navigation() {
        let mut table = create_test_table();
        table.focused_cell = Some(CellPosition::new(1, 1));

        // Test navigation
        table.move_focus_right();
        assert_eq!(table.focused_cell(), Some(CellPosition::new(1, 2)));

        table.move_focus_down();
        assert_eq!(table.focused_cell(), Some(CellPosition::new(2, 2)));

        table.move_focus_left();
        assert_eq!(table.focused_cell(), Some(CellPosition::new(2, 1)));

        table.move_focus_up();
        assert_eq!(table.focused_cell(), Some(CellPosition::new(1, 1)));
    }

    #[test]
    fn test_table_navigation_boundaries() {
        let mut table = create_test_table();

        // Test navigation at boundaries
        table.move_focus_home();
        assert_eq!(table.focused_cell(), Some(CellPosition::new(0, 0)));

        // Try to move up from top-left (should stay)
        table.move_focus_up();
        assert_eq!(table.focused_cell(), Some(CellPosition::new(0, 0)));

        // Try to move left from top-left (should stay)
        table.move_focus_left();
        assert_eq!(table.focused_cell(), Some(CellPosition::new(0, 0)));

        // Move to end
        table.move_focus_end();
        assert_eq!(table.focused_cell(), Some(CellPosition::new(2, 2)));

        // Try to move down from bottom-right (should stay)
        table.move_focus_down();
        assert_eq!(table.focused_cell(), Some(CellPosition::new(2, 2)));

        // Try to move right from bottom-right (should stay)
        table.move_focus_right();
        assert_eq!(table.focused_cell(), Some(CellPosition::new(2, 2)));
    }

    #[test]
    fn test_table_sorting() {
        let mut table = create_test_table();

        // Sort by name column (index 1) ascending
        table.sort_by_column(1, SortDirection::Ascending);

        // Check that rows are sorted by name
        assert_eq!(table.get_row(0).unwrap().get_cell(1).unwrap(), "Alice");
        assert_eq!(table.get_row(1).unwrap().get_cell(1).unwrap(), "Bob");
        assert_eq!(table.get_row(2).unwrap().get_cell(1).unwrap(), "Charlie");

        // Sort descending
        table.sort_by_column(1, SortDirection::Descending);
        assert_eq!(table.get_row(0).unwrap().get_cell(1).unwrap(), "Charlie");
        assert_eq!(table.get_row(1).unwrap().get_cell(1).unwrap(), "Bob");
        assert_eq!(table.get_row(2).unwrap().get_cell(1).unwrap(), "Alice");

        // Check sort configuration
        let sort = table.current_sort().unwrap();
        assert_eq!(sort.column_index, 1);
        assert_eq!(sort.direction, SortDirection::Descending);
    }

    #[test]
    fn test_table_multiple_selection() {
        let mut table = create_test_table().with_selection_mode(TableSelectionMode::MultipleRows);

        // Select multiple rows
        table.select_row(0);
        table.toggle_row_selection(2);

        assert_eq!(table.selected_rows(), &[0, 2]);

        // Toggle off first row
        table.toggle_row_selection(0);
        assert_eq!(table.selected_rows(), &[2]);
    }

    #[test]
    fn test_table_cell_operations() {
        let mut table = create_test_table();

        // Test cell content access
        let content = table.get_cell_content(CellPosition::new(1, 1)).unwrap();
        assert_eq!(content, "Bob");

        // Test cell content modification
        table.set_cell_content(CellPosition::new(1, 1), "Robert".to_string());
        let updated_content = table.get_cell_content(CellPosition::new(1, 1)).unwrap();
        assert_eq!(updated_content, "Robert");
    }

    #[test]
    fn test_table_configuration() {
        let table = Table::new()
            .with_selection_mode(TableSelectionMode::Cell)
            .with_visible_rows(15)
            .with_visible_columns(8)
            .show_headers(false)
            .show_grid(false);

        assert_eq!(table.selection_mode, TableSelectionMode::Cell);
        assert_eq!(table.visible_rows, 15);
        assert_eq!(table.visible_columns, 8);
        assert!(!table.show_headers);
        assert!(!table.show_grid);
    }

    #[tokio::test]
    async fn test_table_rendering() {
        use crate::style::Theme;

        let table = create_test_table();
        let theme = Theme::default();
        let context = RenderContext::new(&theme);

        let result = table.render(&context).await;
        assert!(result.is_ok());

        let vnode = result.unwrap();
        assert_eq!(vnode.tag(), Some("table"));
    }
}
