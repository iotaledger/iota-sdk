def utf8_to_hex(utf8_data):
    return '0x'+utf8_data.encode('utf-8').hex()

def hex_to_utf8(hex_data):
    return bytes.fromhex(hex_data[2:]).decode('utf-8')
