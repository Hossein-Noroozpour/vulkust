###Design pattern rules

- import std first, ffi related second, outer modules third, neighbor last. 
- It is better to avoid big constructor, in big constructors there is several mistakes:
   - Perhaps your mistaking in your design, the structure that you are creating must be divided to other structures.
   - If the previous was not applicable try to create a default version of your structure and then try to break it down 
   to several functions for initializing each data field of structure.