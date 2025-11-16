# Main Page Layout

This document describes the layout of the main page of the application, based on the screenshot provided in `legacy/screenshots/tinavg-mainpage.png`.

## Overall Structure

The page is divided into two main sections:
1.  A header bar at the top.
2.  A main content area below the header.

## Header

The header has a solid, dark teal background. It contains three distinct sections: left, center, and right.

### Left Section

This section is aligned to the left of the header and contains the following elements horizontally:

-   **Score:** A star icon followed by a number, presumably representing the user's score.
-   **Help:** A question mark icon, likely a button to show help or instructions.
-   **Language:** A flag representing the selected language.

### Center Section

-   **Current Word:** A word is displayed in the horizontal and vertical center of the header. The word is underlined, suggesting it's the focus of the current game or puzzle.
-   **User Entered Word**: Below the "Current Word" is a blank space that will hold the user word.

### Right Section

This section is aligned to the right of the header and contains the following elements:

-   **Menu:** A hamburger icon (three horizontal lines), which typically opens a navigation menu or settings.
-   **Alphabet:** An icon showing a capital letter 'A' with a line beneath it, possibly to navigate to an alphabet reference page.

## Main Content Area

The main content area is below the header and has a solid, muted brown background.

### Letter Grid

-   The area is dominated by a grid of letters.
-   The grid is structured in rows and columns (e.g., 3 rows by 4 columns in the example).
-   Each cell in the grid contains a single letter.
-   The letters are displayed in a simple, clean, black font.
-   The grid should contain all the unique letters from the "Current Word" in the header, plus additional random or distractor letters to fill the grid.

## Rules of the Game

-   The user is presented with a word that they must spell correctly by clicking the letters in the grid in the main content area
-   The user will receive 1 point for each correct letter
-   Once the user believe they have spelled the word correctly, they must click a button that will appear next to (or below) the user's guess
-   If the user is incorrect, then they receive no points
-   The user will receive 10 bonus points if they get the word correct on the first try and 2 points less for each subsequent try
-   After 5 tries, the game will display the correct spelling and move on to the next word
