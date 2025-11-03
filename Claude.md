# General Approach
- Attempt to impliment the simplest and minimal approach to a problem
- Do not overcomplicate solutions
- Use Context7 anytime for understanding third party libraries/crates

# Workflow
- After every change you make, run cargo build and cargo test.
- Ensure the build compiles properly, with no warnings, if there are warnings, resolve them.
- Ensure all tests pass.
- Each function/method should have a test.

# Code Style
- Ensure each file is 100 lines of code or less unless it is a specific struct
- Create a struct for each data structure leveraged in the codebase
- Ensure every result or some() is destructured in a match statement where possible
- Do not have .unwrap() in code
- Ensure explicit commenting and documentation where possible that is also professional and minimal
