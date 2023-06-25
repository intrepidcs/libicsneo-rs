from icsneo import *


v = get_version()

print("major: ", v.major())
print("minor: ", v.minor())
print("patch: ", v.patch())
print("metadata: ", v.metadata())
print("build_branch: ", v.build_branch())
print("build_tag: ", v.build_tag())
