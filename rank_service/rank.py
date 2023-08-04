from flask import Flask, request, jsonify
import numpy as np
import faiss
from transformers import AutoTokenizer, AutoModel

app = Flask(__name__)

# Assume you're using the 'bert-base-uncased' model
tokenizer = AutoTokenizer.from_pretrained('bert-base-uncased')
model = AutoModel.from_pretrained('bert-base-uncased')

@app.route('/rank', methods=['POST'])
def rank():
    docs = request.get_json()  # Get the JSON data from the request
    query = request.args.get('query')  # Get the 'query' parameter
    if not isinstance(docs, list):
        return jsonify({'error': 'Payload is not a list'}), 400
    if not all(isinstance(item, str) for item in docs):
        return jsonify({'error': 'All items in the list are not strings'}), 400

    print(docs)

    # Create embeddings for all documents and add them to the FAISS index
    embeddings = []
    for doc in docs:
        tokens = tokenizer.encode_plus(doc, max_length=512, truncation=True, padding='max_length', return_tensors='pt')
        input_ids = tokens['input_ids']
        attention_mask = tokens['attention_mask']
        outputs = model(input_ids, attention_mask=attention_mask)
        doc_embedding = outputs.last_hidden_state[:,-1].detach().numpy()
        embeddings.append(doc_embedding)

    dimension = len(embeddings[0][0])
    index = faiss.IndexFlatL2(dimension)
    v = np.vstack(embeddings)
    index.add(v)

    print(query)

    t = tokenizer(query, return_tensors='pt')
    outputs2 = model(**t)
    query_embedding = outputs2.last_hidden_state[:,-1].detach().numpy()

    # Perform a search with FAISS
    _, I = index.search(query_embedding, 5)

    print(I,I[0])
    

    return str(I[0][0]), 200

if __name__ == '__main__':
    app.run(host='0.0.0.0', port=8081)
