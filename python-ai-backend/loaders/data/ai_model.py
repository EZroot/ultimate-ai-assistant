class AIModel:
    def __init__(self, controlnet_model_id, diffusion_model_id, useinpainting, route):
        self.controlnet_model_id = controlnet_model_id
        self.diffusion_model_id = diffusion_model_id
        self.useinpainting = useinpainting
        self.route = route

    def display_models(self):
        print("Control-Net Model ID:", self.controlnet_model_id)
        print("Diffusion Model ID:", self.diffusion_model_id)
        print("Use Inpainting:", self.useinpainting)
        print("Endpoint :", self.route)
