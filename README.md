# 📋 Automated Team TODO Spreadsheet

Transform a blank Google Sheet into a fully automated, collaborative task manager for your team. This Google Apps Script handles formatting, dropdowns, timestamps, and automatically archives completed tasks every week to keep your workspace clean.

## ✨ Features

* **🚀 One-Click Setup:** Run a single function to generate all sheets, headers, dropdowns, and formatting automatically.
* **⏱️ Auto-Timestamping:** Checking a task automatically logs the exact date and time of completion.
* **🎨 Visual Feedback:** Conditional formatting automatically crosses out completed tasks and highlights high-priority items.
* **🗂️ Weekly Auto-Archive:** A background trigger runs every Sunday night, moving tasks completed more than 24 hours ago into a dedicated monthly archive sheet (e.g., `Archive_October_2023`).
* **📊 Live Dashboard:** A built-in dashboard tracks open tasks, urgencies, and pending completions in real-time.
* **👥 Team Ready:** Built-in data validation dropdowns for Assignees and Priorities to prevent typing errors.

## 🛠️ Prerequisites

* A Google Account
* A blank Google Spreadsheet

## 🚀 Installation & Setup

1. Create a new, blank [Google Sheet](https://sheets.new/).
2. In the top menu, click on **Extensions** > **Apps Script**.
3. Delete any code in the editor and paste the entire content of `Code.gs` (or whatever you named your script file).
4. **Important Customization:** At the top of the script, update the `TEAM_MEMBERS` array with the names of your team members. (e.g., `const TEAM_MEMBERS = ["Alice", "Bob", "Charlie"];`)
5. Save the project by clicking the **Save** icon (💾).
6. In the toolbar, select `SETUP_FULL_INSTALLATION` from the dropdown menu next to the "Run" button.
7. Click **Run**.
8. Google will prompt you to authorize the script. 
   * Click **Review permissions**.
   * Choose your Google Account.
   * You may see a "Google hasn’t verified this app" warning. Click **Advanced** and then **Go to [Project Name] (unsafe)**.
   * Click **Allow**.
9. Return to your Google Sheet. The structure, conditional formatting, dashboard, and background triggers are now fully set up!

## 💡 How to Use

* **Adding Tasks:** Simply type a task in the "Task Description" column on the `TODO` sheet. Select a Priority and Assignee from the dropdown menus.
* **Completing Tasks:** Click the checkbox in the "Status" column. The row will turn gray, strike through the text, and automatically log the completion date.
* **Undoing a Completion:** If you checked a box by mistake, simply uncheck it. The timestamp will clear automatically. (The auto-archive has a built-in 24-hour safety buffer, so recently completed tasks won't instantly disappear).
* **Viewing Stats:** Check the `DASHBOARD` sheet for a real-time overview of your team's workload.

## ⚙️ How the Auto-Archive Works

During the setup process, the script automatically creates a Time-Driven Trigger. 
* **When:** Every Sunday between 11 PM and Midnight.
* **What it does:** It scans the `TODO` sheet for checked tasks. If a task was completed *more than 24 hours ago*, it removes it from the `TODO` sheet and appends it to a monthly archive sheet (creating the month's sheet automatically if it doesn't exist yet).

## 📄 License

This project is open-source and available under the [MIT License](LICENSE). Feel free to fork, modify, and improve it!