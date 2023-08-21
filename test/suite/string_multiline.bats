# bats file_tags=tag:string
@test "string/multiline.lox" {
  run target/debug/lox test/cases/string/multiline.lox

  [ "${lines[0]}" = "1" ]
  [ "${lines[1]}" = "2" ]
  [ "${lines[2]}" = "3" ]
}
