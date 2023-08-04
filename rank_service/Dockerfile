# generate alpine python docker file
FROM python:3.8

# set working directory


WORKDIR /app

# install dependencies

RUN apt-get update && apt-get install -y \
    libopenblas-dev \
    swig

RUN pip install faiss-cpu flask numpy torch transformers 

# copy project
COPY . /app

# run server
EXPOSE 8081

CMD ["python", "rank.py"]
