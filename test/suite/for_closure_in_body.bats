# bats file_tags=tag:for
skip
@test "for/closure_in_body.lox" {
  run target/debug/lox test/cases/for/closure_in_body.lox

  [ "${lines[0]}" = "4" ]
  [ "${lines[1]}" = "1" ]
  [ "${lines[2]}" = "4" ]
  [ "${lines[3]}" = "2" ]
  [ "${lines[4]}" = "4" ]
  [ "${lines[5]}" = "3" ]
}
