# Use an official Node.js image as the base
FROM node:22

# Set the working directory inside the container
WORKDIR /app

# Copy package.json and yarn.lock
COPY package.json yarn.lock ./

# Install dependencies
RUN yarn install

# Copy the rest of the application files
COPY . .

# Build the application
RUN yarn build

# Expose the port that the app will run on
EXPOSE 3000

# Start the application
CMD ["yarn", "dev"]
