--- @class Node
--- @field value number
--- @field next Node|nil

--- A simple linked list implementation in Lua.
--- We use prepending to add new nodes to the list, which is efficient for linked lists.
list = nil

-- Build the chained list by prepending new nodes
list = { next = list, value = 1}
list = { next = list, value = 2}
list = { next = list, value = 3}
list = { next = list, value = 4}

--- Iterates through the list and prints values.
--- This follows the .next pointers until it hits nil.
local l = list
while l do
  print(l.value)
  l = l.next
end