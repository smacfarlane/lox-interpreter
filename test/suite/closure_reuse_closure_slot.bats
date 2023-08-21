# bats file_tags=tag:closure
@test "closure/reuse_closure_slot.lox" {
  run target/debug/lox test/cases/closure/reuse_closure_slot.lox

  [ "${lines[0]}" = "a" ]
}
