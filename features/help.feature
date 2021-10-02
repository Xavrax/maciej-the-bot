Feature: Printing help.txt as a help message on chat
  Scenario: Print help after calling help command
    When command "help" is triggered
    Then message should include "help.txt"