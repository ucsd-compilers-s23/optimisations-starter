mod infra;

// Your tests go here!
success_tests! {
    {
        name: make_vec_succ,
        file: "make_vec.snek",
        input: "5",
        expected: "[0, 0, 0, 0, 0]",
    },
    {
        name: vec_succ,
        file: "vec.snek",
        expected: "[0, 1, 2, 3]",
    },
    {
        name: vec_get_succ,
        file: "vec_get.snek",
        input: "3",
        expected: "3",
    },
    {
        name: linked_list_manipulations,
        file: "linked_list_manipulations.snek",
        expected: "1\n2\n3\n4\n5\n5\n4\n3\n2\n1\nnil"
    },
}

runtime_error_tests! {
    {
        name: make_vec_oom,
        file: "make_vec.snek",
        input: "5",
        heap_size: 5,
        expected: "out of memory",
    },
    {
        name: vec_get_oob,
        file: "vec_get.snek",
        input: "5",
        expected: "",
    }
}

static_error_tests! {}

profile_tests! {
    {
        name: profile_linked_list_manipulations,
        file: "linked_list_manipulations.snek",
        time_trials: 20,
        expected: "1\n2\n3\n4\n5\n5\n4\n3\n2\n1\nnil",
    },
    {
        name: profile_simple_sum,
        file: "simple_sum.snek",
        input: "10",
        expected: "55",
    },
    {
        name: profile_bigloop,
        file: "bigloop.snek",
        input: "100000000",
        expected: "100",
    },
}
