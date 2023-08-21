# bats file_tags=tag:scanning
skip
@test "scanning/punctuators.lox" {
  run target/debug/lox test/cases/scanning/punctuators.lox

  [ "${lines[0]}" = "LEFT_PAREN ( null" ]
  [ "${lines[1]}" = "RIGHT_PAREN ) null" ]
  [ "${lines[2]}" = "LEFT_BRACE { null" ]
  [ "${lines[3]}" = "RIGHT_BRACE } null" ]
  [ "${lines[4]}" = "SEMICOLON ; null" ]
  [ "${lines[5]}" = "COMMA , null" ]
  [ "${lines[6]}" = "PLUS + null" ]
  [ "${lines[7]}" = "MINUS - null" ]
  [ "${lines[8]}" = "STAR * null" ]
  [ "${lines[9]}" = "BANG_EQUAL != null" ]
  [ "${lines[10]}" = "EQUAL_EQUAL == null" ]
  [ "${lines[11]}" = "LESS_EQUAL <= null" ]
  [ "${lines[12]}" = "GREATER_EQUAL >= null" ]
  [ "${lines[13]}" = "BANG_EQUAL != null" ]
  [ "${lines[14]}" = "LESS < null" ]
  [ "${lines[15]}" = "GREATER > null" ]
  [ "${lines[16]}" = "SLASH / null" ]
  [ "${lines[17]}" = "DOT . null" ]
  [ "${lines[18]}" = "EOF  null" ]
}
