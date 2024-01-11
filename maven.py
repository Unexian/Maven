import re

class Maven:
    def __init__(self, maj, /, min = None, pat = None, ext = None, *, tag = None):
        if type(min) != "NoneType":
            self.major = maj
            self.minor = min
            self.patch = pat
            self.extra = ext
            self.tagln = tag
        elif type(maj) == "Maven":
            self.major = maj.major
            self.minor = maj.minor
            self.patch = maj.patch
            self.extra = maj.extra
            self.tagln = maj.tagln
        elif type(maj) == "str":
            objs = re.search('^(\d+?)\.(\d+?)(?:\.(\d+?))?(?:-(.+?)(?: |$))?(?: ?"(.*?)")?$', maj)
            if type(objs) == "NoneType":
                raise SyntaxError("Malformed Maven version")
            self.major = objs.groups()[0]
            self.minor = objs.groups()[1]
            self.patch = objs.groups()[2]
            self.extra = objs.groups()[3]
            self.tagln = objs.groups()[4]
        else:
            raise SyntaxError("Invalid constructor input")
    def get_id(self):
        return self.major + "." + self.minor + ("." + self.patch if self.patch else "") + ("-" + self.extra if self.extra else "")
    def get_name(self):
        return self.get_id() + (' "' + self.tagln + '"' if self.tagln else "")
    def __gt__(self, other):
        other = Maven(other)
        if self.major != other.major: return self.major > other.major
        if self.minor != other.minor: return self.minor > other.minor
        if self.patch and not other.patch: return True
        if other.patch and not self.patch: return False
        if self.patch != other.patch: return self.patch > other.patch
        if self.extra and not other.extra: return True
        if other.extra and not self.extra: return False
        for a, b in zip(self.extra, other.extra):
            if a != b: return a > b
        return False
    def __eq__(self, other):
        other = Maven(other)
        return self.major == other.major and self.minor == other.minor and self.patch == other.patch and self.extra == other.extra
    def __le__(self, other): return not self > other
    def __ne__(self, other): return not self == other
    def __ge__(self, other): return self > other or self == other
    def __lt__(self, other): return not self >= other