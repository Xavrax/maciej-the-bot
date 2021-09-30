Feature: Displaying help.txt as a part of a help message
  Scenario: Display help after running binary with --help flag
    When binary should print commands help
    Then message should include "help.txt"