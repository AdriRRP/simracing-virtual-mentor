if (typeof env.LOAD_ANALYSIS_TEST_DATA !== 'undefined' && env.LOAD_ANALYSIS_TEST_DATA === 'true') {
    // Create a new collection and insert documents
    db.analysis.insert([
        {
            header: {
                _id: UUID("00000000-0000-0000-0000-000000000001"),
                name: "Test Driver",
                date: new ISODate("2024-02-24T20:30:00Z"),
                circuit: "Test Circuit",
            },

            reference: {
                number: 0,
                driver: "Test Driver",
                category: "Test Category",
                car: "Test Car",
                metrics: {
                    speed: [],
                    throttle: [],
                    brake: [],
                    clutch: [],
                    gear: [],
                    rpm: [],
                    distance: [],
                    distance_pct: [],
                    track_temp: [],
                    latitude: [],
                    longitude: [],
                    altitude: [],
                    steering_wheel_angle: [],
                    fuel_level: [],
                    lap_current_lap_time: [],
                }
            },
            target: {
                number: 0,
                driver: "Test Driver 2",
                category: "Test Category 2",
                car: "Test Car 2",
                metrics: {
                    speed: [],
                    throttle: [],
                    brake: [],
                    clutch: [],
                    gear: [],
                    rpm: [],
                    distance: [],
                    distance_pct: [],
                    track_temp: [],
                    latitude: [],
                    longitude: [],
                    altitude: [],
                    steering_wheel_angle: [],
                    fuel_level: [],
                    lap_current_lap_time: [],
                }
            },
            union_distances: [],
            differences: {
                speed: [],
                throttle: [],
                brake: [],
                clutch: [],
                gear: [],
                rpm: [],
                distance: [],
                distance_pct: [],
                track_temp: [],
                latitude: [],
                longitude: [],
                altitude: [],
                steering_wheel_angle: [],
                fuel_level: [],
                lap_current_lap_time: [],
            }
        },
        {
            header: {
                _id: UUID("00000000-0000-0000-0000-000000000002"),
                name: "Test Analysis",
                date: new ISODate("2024-02-25T20:30:00Z"),
                circuit: "Test Circuit 2",
            },

            reference: {
                number: 0,
                driver: "Test Driver",
                category: "Test Category",
                car: "Test Car",
                metrics: {
                    speed: [],
                    throttle: [],
                    brake: [],
                    clutch: [],
                    gear: [],
                    rpm: [],
                    distance: [],
                    distance_pct: [],
                    track_temp: [],
                    latitude: [],
                    longitude: [],
                    altitude: [],
                    steering_wheel_angle: [],
                    fuel_level: [],
                    lap_current_lap_time: [],
                }
            },
            target: {
                number: 0,
                driver: "Test Driver 2",
                category: "Test Category 2",
                car: "Test Car 2",
                metrics: {
                    speed: [],
                    throttle: [],
                    brake: [],
                    clutch: [],
                    gear: [],
                    rpm: [],
                    distance: [],
                    distance_pct: [],
                    track_temp: [],
                    latitude: [],
                    longitude: [],
                    altitude: [],
                    steering_wheel_angle: [],
                    fuel_level: [],
                    lap_current_lap_time: [],
                }
            },
            union_distances: [],
            differences: {
                speed: [],
                throttle: [],
                brake: [],
                clutch: [],
                gear: [],
                rpm: [],
                distance: [],
                distance_pct: [],
                track_temp: [],
                latitude: [],
                longitude: [],
                altitude: [],
                steering_wheel_angle: [],
                fuel_level: [],
                lap_current_lap_time: [],
            }
        },
    ]);
}
