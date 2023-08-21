# bats file_tags=tag:closure
@test "closure/unused_closure.lox" {
  run target/debug/lox test/cases/closure/unused_closure.lox

  [ "${lines[0]}" = "ok" ]
}
