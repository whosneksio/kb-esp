from flask import Flask, request, jsonify
import subprocess

app = Flask(__name__)

PORT = 2598

class hid:
    def __init__(self):
        self.process = None
        self.current_color = None
    
    def change_color(self, new_color):
        if new_color != self.current_color:
            if self.process:
                self.process.terminate()
                self.process.wait()
            self.process = subprocess.Popen(["hid", "--c", new_color])
            self.current_color = new_color

api = hid()

@app.route('/', methods=['GET'])
def check():
    value = request.args.get('v')
    
    if value is not None:
        try:
            int_value = int(value)
            if int_value < 50:
                api.change_color('FF0000') 
                return jsonify({'status': 'success', 'message': int_value, 'color': 'FF0000'}), 200
            elif int_value >= 50 and int_value < 100:
                api.change_color('FFFF00') 
                return jsonify({'status': 'success', 'message': int_value, 'color': 'FFFF00'}), 200
            else:
                api.change_color('00FF00')  
                return jsonify({'status': 'success', 'message': int_value, 'color': '00FF00'}), 200
        except ValueError:
            return jsonify({'status': 'error', 'message': 'invalid value, must be a number.'}), 400
    else:
        return jsonify({'status': 'error', 'message': 'value parameter is required.'}), 400


if __name__ == '__main__':
    app.run(debug=False, port=PORT)
