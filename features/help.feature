Feature: Displaying help.txt as a part of a help message
  Scenario: Display help after running binary with --help flag
    When command "help" is triggered
    Then message should include "help.txt"