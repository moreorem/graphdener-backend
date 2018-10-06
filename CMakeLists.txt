find_package(ArrayFire)
add_executable(graphdener-backend [list your source files here])

# To use Unified backend, do the following.
# Unified backend lets you choose the backend at runtime
target_link_libraries(<my_executable> ArrayFire::af)
