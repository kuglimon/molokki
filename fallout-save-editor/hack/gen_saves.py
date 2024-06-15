import requests
import json
import re
from pathlib import Path

saves = json.loads(Path('saves.json').read_text())

print(saves)

for save in saves:
    name_parts = save["save_name"].split(".")

    r = requests.get(f"https://fallout.fandom.com/wiki/{name_parts[0]}.SSL")
    page_content = r.text
    pattern = 'Name: (.*)<'
    # print(page_content)
    match = re.search(pattern, page_content)
    full_name = name_parts[0].lower()
    if match is not None:
        full_name = match.group(1).strip().replace(" ", "_").lower()

    s = """
#[test]
fn parses_%s_map_save() {
    let decompressed = try_decompress_dat2(%s_SAV.to_vec());
    let (header, variables, scripts) = dat2(&decompressed);

    assert_eq!(header.local_variable_count, %s);
    assert_eq!(header.global_variable_count, %s);

    assert_eq!(variables.local_variables.len(), %s);
    assert_eq!(variables.global_variables.len(), %s);

    assert_eq!(scripts.len(), %s);
}
    """ % (full_name, name_parts[0], save["local_variable_count"], save["global_variable_count"], save["local_variable_count"], save["global_variable_count"], save["script_count"])
    print(s)

