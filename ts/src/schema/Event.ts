import { TypedSchema } from "../TypedSchema.js";
import type { EventAttendanceModeEnumeration } from "../enum/EventAttendanceModeEnumeration.js";
import type { EventStatusType } from "../enum/EventStatusType.js";
import type { Offer } from "./Offer.js";
import type { Organization } from "./Organization.js";
import type { Person } from "./Person.js";
import type { Place } from "./Place.js";
import type { VirtualLocation } from "./VirtualLocation.js";

export interface EventOptions {
	name: string;
	startDate: string;
	location: Place | VirtualLocation | readonly (Place | VirtualLocation)[];
	description?: string | null;
	endDate?: string | null;
	eventAttendanceMode?: EventAttendanceModeEnumeration | null;
	eventStatus?: EventStatusType | null;
	image?: readonly string[] | null;
	offers?: Offer | readonly Offer[] | null;
	organizer?: Organization | Person | null;
	performer?: Person | readonly Person[] | null;
	previousStartDate?: string | null;
}

export class Event extends TypedSchema {
	static readonly schemaType = "Event";

	public readonly name: string;
	public readonly startDate: string;
	public readonly location:
		| Place
		| VirtualLocation
		| readonly (Place | VirtualLocation)[];
	public readonly description: string | null;
	public readonly endDate: string | null;
	public readonly eventAttendanceMode: EventAttendanceModeEnumeration | null;
	public readonly eventStatus: EventStatusType | null;
	public readonly image: readonly string[] | null;
	public readonly offers: Offer | readonly Offer[] | null;
	public readonly organizer: Organization | Person | null;
	public readonly performer: Person | readonly Person[] | null;
	public readonly previousStartDate: string | null;

	constructor(options: EventOptions) {
		super();
		this.name = options.name;
		this.startDate = options.startDate;
		this.location = options.location;
		this.description = options.description ?? null;
		this.endDate = options.endDate ?? null;
		this.eventAttendanceMode = options.eventAttendanceMode ?? null;
		this.eventStatus = options.eventStatus ?? null;
		this.image = options.image ?? null;
		this.offers = options.offers ?? null;
		this.organizer = options.organizer ?? null;
		this.performer = options.performer ?? null;
		this.previousStartDate = options.previousStartDate ?? null;
	}
}
