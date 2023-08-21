# bats file_tags=tag:scanning
skip
@test "scanning/identifiers.lox" {
  run target/debug/lox test/cases/scanning/identifiers.lox

  [ "${lines[0]}" = "IDENTIFIER andy null" ]
  [ "${lines[1]}" = "IDENTIFIER formless null" ]
  [ "${lines[2]}" = "IDENTIFIER fo null" ]
  [ "${lines[3]}" = "IDENTIFIER _ null" ]
  [ "${lines[4]}" = "IDENTIFIER _123 null" ]
  [ "${lines[5]}" = "IDENTIFIER _abc null" ]
  [ "${lines[6]}" = "IDENTIFIER ab123 null" ]
  [ "${lines[7]}" = "IDENTIFIER abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890_ null" ]
  [ "${lines[8]}" = "EOF  null" ]
}
