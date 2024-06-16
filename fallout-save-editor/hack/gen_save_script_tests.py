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

    match = re.search(pattern, page_content)
    full_name = name_parts[0].lower()
    if match is not None:
        full_name = match.group(1).strip().replace(" ", "_").lower()

    s = """
#[test]
fn parses_%s_map_save_scripts() {
    let decompressed = try_decompress_dat2(%s_SAVE.to_vec());
    let (_, _, scripts) = dat2(&decompressed);

    """ % (full_name, name_parts[0])

    for index, script in enumerate(save["scripts"]):
        s += """
    let script = &scripts[%s];

    assert_eq!(script.id, %s);
    assert_eq!(script.local_variable_offset, %s);
    assert_eq!(script.local_variable_count, %s);
    assert_eq!(script.script_type, ScriptTagType::try_from(%s).unwrap());

    """ % (index, script["sid"], script["start_index"], script["variable_count"], script["scr_type"])

    s += """
}
    """

    print(s)

