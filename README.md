Project Design
Main Components
Task Generator: Creates tasks over time
Queue: Stores tasks before execution
Dispatcher: Decides which task goes next
Worker Pool: Executes tasks concurrently
Metrics Collector: Tracks performance
Task Structure

Each task includes:

id
arrival_time
type (CPU or IO)
duration
Scheduling Policy

This project uses: [INSERT YOUR POLICY HERE]
(Example: FIFO, Priority, Shortest Job First)

Reason:
Explain briefly why you chose this policy and what it improves.

Concurrency Tools Used
Threads
Channels OR Arc/Mutex
Explain where and why you used them.
Metrics Collected
Total tasks completed
Makespan (total runtime)
Average wait time
Average turnaround time
Additional metrics:
[Add 2+ of your choice]
Experiments
Experiment A: Balanced Workload
Mix of CPU and IO tasks
Summary of results:
(Write 2–3 sentences about what happened)
Experiment B: Stressed Workload
Example: many CPU-heavy tasks
Summary of results:
(Write 2–3 sentences comparing performance)
Results Summary

Briefly compare both experiments and explain what you learned.

Lessons Learned
What worked well
What was difficult
Any bugs or issues you fixed
How the System Works (Simple Explanation)

Tasks are generated over time and placed into a queue.
The dispatcher selects tasks based on the scheduling policy and sends them to available workers.
Workers process tasks and report completion while the system tracks performance metrics.

Tool Use Disclosure

Tools used:

(Example: ChatGPT, documentation, etc.)

Help received:

(What the tool helped you with)

Accepted advice:

(One thing that worked)

Rejected or fixed advice:

(One thing you had to change)