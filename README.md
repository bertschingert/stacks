# Stacks

This is a Linux tool that prints out the kernel stack trace of each process in the system.
Processes with the same stack are aggregated, and sorted from most popular to least popular stack.

Example output, on an idle system:
```
# ./stacks
36
kworker/0:0H-events_highpri kworker/1:0H-events_highpri kworker/2:0H-events_highpri kworker/3:0H-events_highpri kworker/4:0H-events_highpri kworker/5:0H-events_highpri kworker/6:0H-events_highpri kworker/7:0-mm_percpu_wq kworker/7:0H-events_highpri kworker/5:1-mm_percpu_wq kworker/1:1-mm_percpu_wq kworker/2:1-mm_percpu_wq kworker/5:1H-kblockd kworker/3:1-mm_percpu_wq kworker/6:1-events kworker/u17:0 kworker/2:1H-kblockd kworker/4:1H-kblockd kworker/0:1H-kblockd kworker/1:1H-kblockd kworker/7:1H-kblockd kworker/3:1H-kblockd kworker/6:1H-kblockd kworker/7:2 kworker/6:2-events kworker/0:2-mm_percpu_wq kworker/0:0 kworker/4:1-events kworker/3:2 kworker/4:2-cgroup_destroy kworker/2:0 kworker/1:2 kworker/5:2 kworker/u16:3-events_unbound kworker/u16:1-events_unbound kworker/u16:2-events_unbound
[<0>] worker_thread+0x19d/0x3a0
[<0>] kthread+0xe8/0x120
[<0>] ret_from_fork+0x34/0x50
[<0>] ret_from_fork_asm+0x1b/0x30

32
ksoftirqd/0 migration/0 idle_inject/0 cpuhp/0 cpuhp/1 idle_inject/1 migration/1 ksoftirqd/1 cpuhp/2 idle_inject/2 migration/2 ksoftirqd/2 cpuhp/3 idle_inject/3 migration/3 ksoftirqd/3 cpuhp/4 idle_inject/4 migration/4 ksoftirqd/4 cpuhp/5 idle_inject/5 migration/5 ksoftirqd/5 cpuhp/6 idle_inject/6 migration/6 ksoftirqd/6 cpuhp/7 idle_inject/7 migration/7 ksoftirqd/7
[<0>] smpboot_thread_fn+0x14b/0x1d0
[<0>] kthread+0xe8/0x120
[<0>] ret_from_fork+0x34/0x50
[<0>] ret_from_fork_asm+0x1b/0x30

[...]

1
systemd-journal
[<0>] jbd2_log_wait_commit+0xd8/0x140 [jbd2]
[<0>] ext4_sync_file+0x1d3/0x380 [ext4]
[<0>] __x64_sys_fsync+0x3b/0x70
[<0>] do_syscall_64+0x64/0xe0
[<0>] entry_SYSCALL_64_after_hwframe+0x6e/0x76

1
kswapd0
[<0>] kswapd+0x3d8/0x400
[<0>] kthread+0xe8/0x120
[<0>] ret_from_fork+0x34/0x50
[<0>] ret_from_fork_asm+0x1b/0x30
```

## Credits

This is inspired by a bash one-liner that I learned about from the talk:
"Debugging ZFS: State of the Art on Linux" by Tom Caputi. 
https://www.youtube.com/watch?v=JoD_Kmqnkgg
