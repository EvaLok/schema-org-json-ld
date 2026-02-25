<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

enum EventAttendanceModeEnumeration: string {
	case OfflineEventAttendanceMode = 'https://schema.org/OfflineEventAttendanceMode';
	case OnlineEventAttendanceMode = 'https://schema.org/OnlineEventAttendanceMode';
	case MixedEventAttendanceMode = 'https://schema.org/MixedEventAttendanceMode';
}
