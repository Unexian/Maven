class Maven {
	constructor(major, minor = undefined, patch = undefined, extra = undefined, tagln = undefined) {
		if (minor !== undefined) {
			this.major = major
			this.minor = minor
			this.patch = patch
			this.extra = extra
			this.tagln = tagln
		} else if (typeof major == "string") {
			let objs = /^(\d+?)\.(\d+?)(?:\.(\d+?))?(?:-(.+?)(?: |$))?(?: ?"(.*?)")?$/.exec(major)
			if (objs === null) { throw SyntaxError("Malformed Maven version") }
			this.major = objs[1]
			this.minor = objs[2]
			this.patch = objs[3]
			this.extra = objs[4]
			this.tagln = objs[5]
		} else if (major instanceof Maven) {
			this.major = major.major
			this.minor = major.minor
			this.patch = major.patch
			this.extra = major.extra
			this.tagln = major.tagln
		} else {
			throw new SyntaxError("Invalid constructor input")
		}
	}
	getId() {
		return this.major + "." + this.minor + this.patch ? "." + this.patch : "" + this.extra ? "-" + this.extra : ""
	}
	getName() {
		return this.getVersionId() + this.tagln ? ' "' + this.tagln + '"' : ""
	}
	gt(other) {
		other = new Maven(other)
		if (this.major != other.major) { return this.major > other.major }
		if (this.minor != other.minor) { return this.minor > other.minor }
		if (! this.patch && !! other.patch) { return true }
		if (! other.patch && !! this.patch) { return false }
		if (this.patch != other.patch) { return this.patch > other.patch }
		if (! this.extra && !! other.extra) { return true }
		if (! other.extra && !! this.extra) { return false }
		if (this.extra != other.extra) { return this.extra > other.extra }
		return false
	}
	eq(other) {
		other = new Maven(other)
		return this.major == other.major && this.minor == other.minor && this.patch == other.patch && this.extra == other.extra && this.tagln == other.tagln
	}
	gte(other) { return this.eq(other) || this.gt(other) }
	neq(other) { return !this.eq(other) }
	lte(other) { return !this.gt(other) }
	lt(other) { return !this.gte(other) }
}