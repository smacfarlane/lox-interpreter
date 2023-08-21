# bats file_tags=tag:closure
@test "closure/unused_later_closure.lox" {
  run target/debug/lox test/cases/closure/unused_later_closure.lox

  [ "${lines[0]}" = "a" ]
}
