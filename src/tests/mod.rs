// TODO: test that numeric types (int, float, decimal, and timestamp)
//       must always end with NUM_STOP = {}[](),/\"\'\ \t\n\r\v\f\EOF
//       to align with the Ion spec

mod literals;
mod parsing;
mod queries;