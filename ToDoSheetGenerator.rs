/**
 * GLOBAL CONFIGURATION
 */
const SHEET_TODO = "TODO";
const SHEET_DASHBOARD = "DASHBOARD";
// EDIT YOUR TEAM NAMES HERE
const TEAM_MEMBERS = ["Mario", "Luigi"]; 
const PRIORITIES = ["High", "Medium", "Low"];

// COLUMN INDICES (1-based index for Sheet API usage)
const COL_CHECK = 1;    // A
const COL_TASK = 2;     // B
const COL_PRIO = 3;     // C
const COL_WHO = 4;      // D
const COL_DATE = 5;     // E

/**
 * 1. FULL INSTALLATION FUNCTION (Run this function once!)
 * Creates sheets, headers, formatting, and SETS UP THE AUTOMATIC TRIGGER.
 */
function SETUP_FULL_INSTALLATION() {
  const ss = SpreadsheetApp.getActiveSpreadsheet();
  
  // --- A. SETUP TODO SHEET ---
  let todoSheet = ss.getSheetByName(SHEET_TODO);
  if (!todoSheet) {
    todoSheet = ss.insertSheet(SHEET_TODO);
  } else {
    todoSheet.clear(); // Resets content if it already exists
  }
  
  // Headers
  const headers = [["Status", "Task Description", "Priority", "Assignee", "Completion Date"]];
  todoSheet.getRange(1, 1, 1, 5).setValues(headers).setFontWeight("bold").setBackground("#d9ead3");
  
  // Checkboxes (Column A)
  todoSheet.getRange("A2:A1000").insertCheckboxes();
  
  // Priority Validation (Column C)
  const rulePrio = SpreadsheetApp.newDataValidation().requireValueInList(PRIORITIES).build();
  todoSheet.getRange("C2:C1000").setDataValidation(rulePrio);
  
  // Assignee Validation (Column D)
  const ruleWho = SpreadsheetApp.newDataValidation().requireValueInList(TEAM_MEMBERS).build();
  todoSheet.getRange("D2:D1000").setDataValidation(ruleWho);
  
  // Column widths for aesthetics
  todoSheet.setColumnWidth(COL_CHECK, 50);
  todoSheet.setColumnWidth(COL_TASK, 400);
  
  // --- B. CONDITIONAL FORMATTING ---
  const range = todoSheet.getRange("B2:E1000"); 
  
  // Rule: Completed -> Strikethrough and gray text
  const ruleDone = SpreadsheetApp.newConditionalFormatRule()
    .whenFormulaSatisfied('=$A2=TRUE')
    .setStrikethrough(true)
    .setFontColor("#999999")
    .setRanges([range])
    .build();
    
  // Rule: High Priority -> Light Red background
  const rangePrio = todoSheet.getRange("C2:C1000");
  const ruleHigh = SpreadsheetApp.newConditionalFormatRule()
    .whenTextEqualTo("High")
    .setBackground("#f4cccc")
    .setRanges([rangePrio])
    .build();
    
  // Rule: Medium Priority -> Light Yellow background
  const ruleMed = SpreadsheetApp.newConditionalFormatRule()
    .whenTextEqualTo("Medium")
    .setBackground("#fff2cc")
    .setRanges([rangePrio])
    .build();

  todoSheet.setConditionalFormatRules([ruleDone, ruleHigh, ruleMed]);

  // --- C. SETUP DASHBOARD ---
  let dashSheet = ss.getSheetByName(SHEET_DASHBOARD);
  if (!dashSheet) {
    dashSheet = ss.insertSheet(SHEET_DASHBOARD);
  }
  dashSheet.clear();
  
  dashSheet.getRange("A1").setValue("📊 TEAM DASHBOARD").setFontSize(14).setFontWeight("bold");
  
  const stats = [
    ["Current Metrics", "Value"],
    ["Total Open Tasks", `=COUNTIF('${SHEET_TODO}'!A:A, FALSE)`],
    ["Urgent (High Priority)", `=COUNTIFS('${SHEET_TODO}'!A:A, FALSE, '${SHEET_TODO}'!C:C, "High")`],
    ["Completed (Pending Archive)", `=COUNTIF('${SHEET_TODO}'!A:A, TRUE)`]
  ];
  
  dashSheet.getRange(3, 1, 4, 2).setValues(stats);
  dashSheet.getRange("A3:B3").setFontWeight("bold").setBackground("#cfe2f3");

  // --- D. AUTOMATIC TRIGGER CONFIGURATION ---
  setupTrigger_(); // Calls the internal helper function
  
  SpreadsheetApp.getUi().alert("Installation complete! Structure created and Trigger activated.");
}

/**
 * Helper function to create the trigger.
 * Deletes existing triggers for 'weeklyArchive' to avoid duplicates, then creates a new one.
 */
function setupTrigger_() {
  const functionName = 'weeklyArchive';
  
  // 1. Search and delete existing triggers
  const triggers = ScriptApp.getProjectTriggers();
  for (let i = 0; i < triggers.length; i++) {
    if (triggers[i].getHandlerFunction() === functionName) {
      ScriptApp.deleteTrigger(triggers[i]);
    }
  }
  
  // 2. Create a new trigger: Every Sunday around 11 PM
  ScriptApp.newTrigger(functionName)
    .timeBased()
    .everyWeeks(1)
    .onWeekDay(ScriptApp.WeekDay.SUNDAY)
    .atHour(23)
    .create();
}

/**
 * 2. AUTOMATIC TRACKING (Simple onEdit Trigger)
 * Note: onEdit is a simple trigger, it runs automatically without installation.
 */
function onEdit(e) {
  const range = e.range;
  const sheet = range.getSheet();
  
  // Check: TODO Sheet + Checkbox Column + Row > 1
  if (sheet.getName() === SHEET_TODO && range.getColumn() === COL_CHECK && range.getRow() > 1) {
    const dateCell = sheet.getRange(range.getRow(), COL_DATE);
    
    if (e.value === "TRUE") {
      // Insert current date and time
      dateCell.setValue(new Date());
    } else {
      // If unchecked, clear the date
      dateCell.clearContent();
    }
  }
}

/**
 * 3. AUTOMATIC ARCHIVING (Executed by the Trigger created above)
 * Archives tasks completed more than 24 hours ago.
 */
function weeklyArchive() {
  const ss = SpreadsheetApp.getActiveSpreadsheet();
  const sheet = ss.getSheetByName(SHEET_TODO);
  const data = sheet.getDataRange().getValues();
  const now = new Date();
  const safeTime = 24 * 60 * 60 * 1000; // 24 hours in milliseconds
  
  // Reverse loop to allow row deletion without index shifting
  for (let i = data.length - 1; i >= 1; i--) {
    const row = data[i];
    const isChecked = row[COL_CHECK - 1]; 
    const completedDate = row[COL_DATE - 1];
    
    // Logic: Is it checked? Is the date valid? Is it older than 24h?
    if (isChecked === true && completedDate instanceof Date) {
      const timeDiff = now - completedDate;
      
      if (timeDiff > safeTime) {
        // --- ARCHIVING PROCESS ---
        
        // Determine Month Sheet Name (e.g., Archive_October_2023)
        const monthNames = ["January", "February", "March", "April", "May", "June",
                            "July", "August", "September", "October", "November", "December"];
        const year = completedDate.getFullYear();
        const month = monthNames[completedDate.getMonth()];
        const archiveSheetName = `Archive_${month}_${year}`;
        
        let archiveSheet = ss.getSheetByName(archiveSheetName);
        if (!archiveSheet) {
          archiveSheet = ss.insertSheet(archiveSheetName);
          // Archive Headers (Matches TODO columns)
          archiveSheet.appendRow(["Task Description", "Priority", "Assignee", "Completion Date"]);
          archiveSheet.getRange("A1:D1").setFontWeight("bold");
        }
        
        // Data to copy (Task, Priority, Assignee, Date) - Excluding Checkbox
        const dataToArchive = [row[1], row[2], row[3], row[4]];
        
        archiveSheet.appendRow(dataToArchive);
        sheet.deleteRow(i + 1);
      }
    }
  }
}