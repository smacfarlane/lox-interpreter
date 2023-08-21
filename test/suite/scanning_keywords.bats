# bats file_tags=tag:scanning
skip
@test "scanning/keywords.lox" {
  run target/debug/lox test/cases/scanning/keywords.lox

  [ "${lines[0]}" = "AND and null" ]
  [ "${lines[1]}" = "CLASS class null" ]
  [ "${lines[2]}" = "ELSE else null" ]
  [ "${lines[3]}" = "FALSE false null" ]
  [ "${lines[4]}" = "FOR for null" ]
  [ "${lines[5]}" = "FUN fun null" ]
  [ "${lines[6]}" = "IF if null" ]
  [ "${lines[7]}" = "NIL nil null" ]
  [ "${lines[8]}" = "OR or null" ]
  [ "${lines[9]}" = "RETURN return null" ]
  [ "${lines[10]}" = "SUPER super null" ]
  [ "${lines[11]}" = "THIS this null" ]
  [ "${lines[12]}" = "TRUE true null" ]
  [ "${lines[13]}" = "VAR var null" ]
  [ "${lines[14]}" = "WHILE while null" ]
  [ "${lines[15]}" = "EOF  null" ]
}
