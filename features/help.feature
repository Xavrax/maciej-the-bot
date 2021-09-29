Feature: Displaying help.txt as a part of a help message
  Scenario: Display help after running binary with --help flag
    When binary is ran with "--help" flag
    Then message should include "help.txt"