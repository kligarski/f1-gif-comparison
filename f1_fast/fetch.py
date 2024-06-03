import fastf1, sys

LAP_STATS = [
    "LapTime",
    "Sector1Time", 
    "Sector2Time", 
    "Sector3Time", 
    "Sector1SessionTime", 
    "Sector2SessionTime", 
    "Sector3SessionTime"
]

DRIVER_STATS = [
    "BroadcastName", 
    "TeamName", 
    "TeamColor"
]

TELEMETRY_STATS = [
    "SessionTime", 
    "X", 
    "Y", 
    "Speed", 
    "RelativeDistance"
]

USAGE_ERROR = 1
SESSION_LOAD_ERROR = 2
DRIVER_DATA_ERROR = 3
LAP_DATA_ERROR = 4
TELEMETRY_DATA_ERROR = 5

DRIVER_DATA_FILE = "driver{}_data.json"
LAP_DATA_FILE = "lap{}_data.json"
TELEMETRY_DATA_FILE = "telemetry{}_data.json"

try:
    # Format: python fetch.py <framerate> <year> <country> <driver1> <driver2> 
    framerate = int(sys.argv[1])
    year = int(sys.argv[2])
    country = sys.argv[3]
    driver1 = sys.argv[4]
    driver2 = sys.argv[5]
except:
    print("Usage: python fetch.py <framerate> <year> <country> <driver1> <driver2>")
    exit(USAGE_ERROR)

try: 
    session = fastf1.get_session(year, country, 'Q')
    session.load()
except:
    print("Unable to fetch session data")
    exit(SESSION_LOAD_ERROR)

try:
    driver1_data = session.get_driver(driver1)[DRIVER_STATS]
    driver2_data = session.get_driver(driver2)[DRIVER_STATS]

    with open(DRIVER_DATA_FILE.format(1), "w") as file:
        driver1_data.to_json(file)
    
    with open(DRIVER_DATA_FILE.format(2), "w") as file:
        driver2_data.to_json(file)
except:
    print("Unable to fetch or export driver data")
    exit(DRIVER_DATA_ERROR)

try:
    lap1_data = session.laps.pick_driver(driver1).pick_fastest()[LAP_STATS]
    lap2_data = session.laps.pick_driver(driver2).pick_fastest()[LAP_STATS]

    with open(LAP_DATA_FILE.format(1), "w") as file:
        lap1_data.to_json(file)
    
    with open(LAP_DATA_FILE.format(2), "w") as file:
        lap2_data.to_json(file)
except:
    print("Unable to fetch or export lap data")
    exit(LAP_DATA_ERROR)
    
try:
    telemetry1_data = session.laps.pick_driver(driver1).pick_fastest().get_telemetry(frequency=framerate)[TELEMETRY_STATS]
    telemetry2_data = session.laps.pick_driver(driver2).pick_fastest().get_telemetry(frequency=framerate)[TELEMETRY_STATS]

    telemetry1_data.X = telemetry1_data.X.map(lambda x: int(x)) 
    telemetry1_data.Y = telemetry1_data.Y.map(lambda x: int(x)) 

    telemetry2_data.X = telemetry2_data.X.map(lambda x: int(x)) 
    telemetry2_data.Y = telemetry2_data.Y.map(lambda x: int(x)) 

    with open(TELEMETRY_DATA_FILE.format(1), "w") as file:
        telemetry1_data.to_json(file, orient="records")
    
    with open(TELEMETRY_DATA_FILE.format(2), "w") as file:
        telemetry2_data.to_json(file, orient="records")
except:
    print("Unable to fetch or export telemetry data")
    exit(TELEMETRY_DATA_ERROR)