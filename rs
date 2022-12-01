public async Task<ActionResult<ServiceResponse<List<GetClinic_AssistantDto>>>> Get()
        {
            return Ok(await _clinic_AssistantService.GetAllClinic_Assistants());
        }
        [HttpDelete("{id}")]
