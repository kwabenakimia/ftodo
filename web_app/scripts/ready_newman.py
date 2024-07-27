import json


with open("./ftodo_items.postman_collection_create_1_and_2.json", "r") as file:
    newman_data = json.loads(file.read())

with open("./fresh_token.json", "r") as file:
    fresh_token_data = json.loads(file.read())
# there are two tests so we swap the apikey for 
newman_data['item'][0]['request']['auth']['apikey']['value'] = fresh_token_data['token']
newman_data['item'][1]['request']['auth']['apikey']['value'] = fresh_token_data['token']

# print(newman_data['item'][0]['request']['auth']['apikey']['value'])
# print(newman_data['item'][1]['request']['auth']['apikey']['value'])


with open("./test_newman.json", "w") as file:
    file.write(json.dumps(newman_data))
