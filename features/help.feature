Feature: Printing help message on chat

  Scenario: Print user help after calling help command
    When command `help` is triggered
    Then message should include help from correct file

  Scenario: Print operator help after calling operator help command
    When operator command `help` is triggered
    Then message should include help from correct file