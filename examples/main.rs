use rust_ti;
use std::time::Instant;

fn main() {
    let now = Instant::now();

    let open = vec![
        4308.32, 4352.61, 4366.29, 4365.33, 4442.7, 4396.11, 4380.01, 4355.4, 4354.17, 4344.84,
        4337.36, 4367.48, 4374.94, 4422.44, 4449.45, 4442.04, 4422.62, 4404.54, 4394.23, 4415.55,
        4467.69, 4491.5, 4514.61, 4508.86, 4521.78, 4563.87, 4554.38, 4550.16, 4543.39, 4555.19,
        4558.96, 4598.26, 4565.75, 4584.82, 4578.83, 4550.93, 4494.27, 4513.96, 4491.58, 4498.03,
        4501.57, 4487.16, 4450.69, 4458.13, 4478.87, 4433.79, 4416.32, 4344.88, 4380.28, 4415.33,
        4396.44, 4455.16, 4389.38, 4426.03, 4432.75, 4500.34, 4517.01, 4530.85, 4510.06, 4490.35,
        4434.55, 4451.3, 4480.98, 4473.27, 4462.65, 4487.78, 4497.98, 4445.13, 4445.41, 4452.81,
        4374.36, 4341.74, 4310.62, 4312.88, 4282.63, 4269.65, 4328.18, 4284.52, 4269.75, 4233.83,
        4259.31, 4234.79, 4289.02, 4339.75, 4366.59, 4380.94, 4360.49, 4342.37, 4345.23, 4357.35,
        4321.36, 4273.85, 4210.4, 4235.79, 4232.42, 4175.99, 4152.93, 4139.39, 4171.33, 4201.27,
        4268.26, 4334.23, 4364.27, 4366.21, 4384.37, 4391.41, 4364.15, 4406.66, 4458.97, 4505.3,
        4497.08, 4509.55, 4511.7, 4538.77, 4553.04, 4555.84, 4554.86, 4545.55, 4571.84, 4554.87,
        4559.43, 4564.37, 4557.25, 4586.23, 4568.84, 4576.2, 4593.39, 4618.3, 4646.2, 4721.04,
        4714.23, 4725.58, 4743.72, 4764.73, 4724.29, 4753.92, 4758.86, 4773.45, 4786.44, 4782.88,
        4745.2, 4725.07, 4697.42, 4690.57, 4703.7, 4741.93, 4759.94, 4792.13, 4791.18, 4772.35,
        4739.13, 4760.1, 4796.28, 4853.42, 4856.8, 4888.56, 4886.66, 4888.91, 4892.95, 4925.89,
        4899.19, 4861.11, 4916.06, 4957.19, 4950.16, 4973.05, 4995.16, 5004.17, 5026.83, 4967.94,
        4976.44, 5003.14, 5031.13, 4989.32, 4963.03, 5038.83, 5100.92, 5093.0, 5074.6, 5067.2,
        5085.36, 5098.51, 5130.99, 5110.52, 5108.03, 5132.38, 5164.46, 5111.96, 5134.3, 5173.49,
        5175.14, 5123.31, 5154.77, 5139.09, 5181.69, 5253.43, 5242.48, 5219.52, 5228.85, 5226.31,
        5248.03, 5257.97, 5204.29, 5194.37, 5244.05, 5158.95, 5211.37, 5217.03, 5167.88, 5172.95,
        5171.51, 5149.67, 5064.59, 5068.97, 5031.52, 5005.44, 4987.33, 5028.85, 5084.86, 5019.88,
        5084.65, 5114.13, 5103.78, 5029.03, 5049.32, 5122.78, 5142.42, 5187.2, 5168.98, 5189.03,
        5225.49, 5233.08, 5221.1, 5263.26, 5310.07, 5303.1, 5305.35, 5298.69, 5319.28, 5340.26,
        5281.45, 5315.91, 5278.73, 5259.77, 5243.21, 5297.15, 5278.24, 5314.48, 5357.8, 5343.81,
        5341.22, 5353.0,
    ];

    let high = vec![
        4340.13, 4375.37, 4391.82, 4439.2, 4448.47, 4400.15, 4386.22, 4382.25, 4366.55, 4362.06,
        4384.42, 4390.35, 4398.39, 4458.48, 4456.46, 4454.06, 4422.62, 4440.39, 4412.6, 4443.64,
        4488.34, 4517.38, 4527.76, 4532.85, 4562.3, 4578.43, 4564.74, 4555.0, 4563.41, 4580.62,
        4582.47, 4607.07, 4590.16, 4594.22, 4584.62, 4550.93, 4519.49, 4540.34, 4519.84, 4503.31,
        4502.44, 4527.37, 4476.23, 4490.33, 4478.87, 4449.95, 4421.17, 4381.82, 4407.55, 4418.59,
        4443.18, 4458.3, 4418.46, 4439.56, 4500.14, 4521.65, 4532.26, 4541.25, 4514.29, 4490.35,
        4457.81, 4473.53, 4490.77, 4487.11, 4479.39, 4511.99, 4497.98, 4466.36, 4449.85, 4461.03,
        4375.7, 4357.4, 4338.51, 4313.01, 4292.07, 4317.27, 4333.15, 4300.58, 4281.15, 4268.5,
        4267.13, 4324.1, 4341.73, 4385.46, 4378.64, 4385.85, 4377.1, 4383.33, 4393.57, 4364.2,
        4339.54, 4276.56, 4255.84, 4259.38, 4232.42, 4183.6, 4156.7, 4177.47, 4195.55, 4245.64,
        4319.72, 4373.62, 4372.21, 4386.26, 4391.2, 4393.4, 4418.03, 4421.76, 4508.67, 4521.17,
        4511.99, 4520.12, 4557.11, 4542.14, 4568.43, 4560.31, 4560.52, 4568.14, 4587.64, 4569.89,
        4599.39, 4572.37, 4578.56, 4590.74, 4590.92, 4609.23, 4623.71, 4643.93, 4709.69, 4738.57,
        4725.53, 4749.52, 4768.69, 4778.01, 4748.71, 4772.94, 4784.72, 4785.39, 4793.3, 4788.43,
        4754.33, 4729.29, 4726.78, 4721.49, 4764.54, 4765.47, 4790.8, 4798.5, 4802.4, 4782.34,
        4744.23, 4785.79, 4842.07, 4868.41, 4866.48, 4903.68, 4898.15, 4906.69, 4929.31, 4931.09,
        4906.75, 4906.97, 4975.29, 4957.19, 4957.77, 4999.89, 5000.4, 5030.06, 5048.39, 4971.3,
        5002.52, 5032.72, 5038.7, 4993.71, 4983.21, 5094.39, 5111.06, 5097.66, 5080.69, 5077.37,
        5104.99, 5140.33, 5149.67, 5114.54, 5127.97, 5165.62, 5189.26, 5124.66, 5179.87, 5179.14,
        5176.85, 5136.86, 5175.6, 5180.31, 5226.19, 5261.1, 5246.09, 5229.09, 5235.16, 5249.26,
        5264.85, 5263.95, 5208.34, 5228.75, 5256.59, 5222.18, 5219.57, 5224.81, 5178.43, 5211.78,
        5175.03, 5168.43, 5079.84, 5077.96, 5056.66, 5019.02, 5038.84, 5076.12, 5089.48, 5057.75,
        5114.62, 5123.49, 5110.83, 5096.12, 5073.21, 5139.12, 5181.0, 5200.23, 5191.95, 5215.3,
        5239.66, 5237.26, 5250.37, 5311.76, 5325.49, 5305.45, 5325.32, 5324.32, 5323.18, 5341.88,
        5311.65, 5315.91, 5282.27, 5260.21, 5280.33, 5302.11, 5298.8, 5354.16, 5362.35, 5375.08,
        5365.79, 5375.95,
    ];

    let low = vec![
        4304.37, 4349.31, 4337.85, 4362.6, 4407.44, 4367.19, 4360.14, 4351.82, 4341.34, 4328.08,
        4335.0, 4360.22, 4371.97, 4422.44, 4442.29, 4436.61, 4385.05, 4397.4, 4389.92, 4408.46,
        4463.23, 4489.36, 4499.56, 4504.9, 4514.59, 4557.48, 4527.56, 4535.79, 4541.29, 4552.42,
        4547.58, 4528.56, 4564.01, 4573.14, 4567.53, 4505.75, 4485.54, 4474.55, 4491.15, 4464.39,
        4461.33, 4457.92, 4443.98, 4453.44, 4432.19, 4403.55, 4364.83, 4335.31, 4360.3, 4382.77,
        4396.44, 4375.55, 4356.29, 4414.98, 4431.68, 4493.59, 4507.39, 4501.35, 4496.01, 4442.38,
        4430.46, 4448.38, 4467.89, 4456.83, 4453.52, 4478.69, 4447.21, 4442.11, 4416.61, 4401.38,
        4329.17, 4316.49, 4302.7, 4265.98, 4238.63, 4264.38, 4274.86, 4260.21, 4216.45, 4220.48,
        4225.91, 4219.55, 4283.79, 4339.64, 4345.34, 4325.43, 4311.97, 4342.37, 4337.54, 4303.84,
        4269.69, 4223.03, 4189.22, 4219.43, 4181.42, 4127.9, 4103.78, 4132.94, 4153.12, 4197.74,
        4268.26, 4334.23, 4347.53, 4355.41, 4359.76, 4343.94, 4353.34, 4393.82, 4458.97, 4495.31,
        4487.83, 4499.66, 4510.36, 4525.51, 4545.05, 4552.8, 4546.32, 4540.51, 4547.15, 4537.24,
        4554.71, 4546.72, 4551.68, 4546.5, 4565.22, 4574.06, 4593.39, 4608.09, 4643.23, 4694.34,
        4704.69, 4725.58, 4743.72, 4697.82, 4708.35, 4736.77, 4758.45, 4768.9, 4780.98, 4751.99,
        4722.67, 4699.71, 4687.53, 4682.11, 4699.82, 4730.35, 4756.2, 4739.58, 4768.98, 4747.12,
        4714.82, 4740.57, 4785.87, 4844.05, 4844.37, 4865.94, 4869.34, 4881.47, 4887.4, 4916.27,
        4845.15, 4853.52, 4907.99, 4918.09, 4934.88, 4969.05, 4987.09, 5000.34, 5016.83, 4920.31,
        4956.45, 4999.44, 4999.52, 4955.02, 4946.0, 5038.83, 5081.46, 5068.91, 5057.29, 5058.35,
        5061.89, 5094.16, 5127.18, 5056.82, 5092.22, 5128.21, 5117.5, 5091.14, 5114.48, 5151.88,
        5123.3, 5104.35, 5145.47, 5131.59, 5171.55, 5240.66, 5229.87, 5216.09, 5203.42, 5213.92,
        5245.82, 5229.2, 5184.05, 5194.37, 5146.06, 5157.21, 5197.35, 5160.78, 5138.7, 5138.77,
        5107.94, 5052.47, 5039.83, 5007.25, 5001.89, 4953.56, 4969.4, 5027.96, 5047.02, 4990.58,
        5073.14, 5088.65, 5035.31, 5013.45, 5011.05, 5101.22, 5142.42, 5178.96, 5165.86, 5180.41,
        5209.68, 5211.16, 5217.98, 5263.26, 5296.19, 5283.59, 5302.4, 5297.87, 5286.01, 5256.93,
        5278.39, 5280.89, 5262.7, 5222.1, 5191.68, 5234.32, 5257.63, 5297.64, 5335.36, 5331.33,
        5331.52, 5327.25,
    ];

    let close = vec![
        4338.93, 4369.01, 4372.59, 4425.84, 4409.59, 4388.71, 4365.69, 4381.89, 4348.33, 4328.82,
        4378.41, 4376.86, 4396.44, 4450.38, 4455.59, 4446.82, 4411.59, 4398.95, 4409.53, 4439.26,
        4472.16, 4510.04, 4505.42, 4522.79, 4554.98, 4565.72, 4534.87, 4536.34, 4554.64, 4567.46,
        4566.75, 4537.41, 4582.23, 4588.96, 4576.73, 4513.39, 4501.89, 4478.03, 4518.44, 4499.38,
        4467.71, 4468.83, 4464.05, 4489.72, 4437.86, 4404.33, 4370.36, 4369.71, 4399.77, 4387.55,
        4436.01, 4376.31, 4405.71, 4433.31, 4497.63, 4514.87, 4507.66, 4515.77, 4496.83, 4465.48,
        4451.14, 4457.49, 4487.46, 4461.9, 4467.44, 4505.1, 4450.32, 4453.53, 4443.95, 4402.2,
        4330.0, 4320.06, 4337.44, 4273.53, 4274.51, 4299.7, 4288.05, 4288.39, 4229.45, 4263.75,
        4258.19, 4308.5, 4335.66, 4358.24, 4376.95, 4349.61, 4327.78, 4373.63, 4373.2, 4314.6,
        4278.0, 4224.16, 4217.04, 4247.68, 4186.77, 4137.23, 4117.37, 4166.82, 4193.8, 4237.86,
        4317.78, 4358.34, 4365.98, 4378.38, 4382.78, 4347.35, 4415.24, 4411.55, 4495.7, 4502.88,
        4508.24, 4514.02, 4547.38, 4538.19, 4556.62, 4559.34, 4550.43, 4554.89, 4550.58, 4567.8,
        4594.63, 4569.78, 4567.18, 4549.34, 4585.59, 4604.37, 4622.44, 4643.7, 4707.09, 4719.55,
        4719.19, 4740.56, 4768.37, 4698.35, 4746.75, 4754.63, 4774.75, 4781.58, 4783.35, 4769.83,
        4742.83, 4704.81, 4688.68, 4697.24, 4763.54, 4756.5, 4783.45, 4780.24, 4783.83, 4765.98,
        4739.21, 4780.94, 4839.81, 4850.43, 4864.6, 4868.55, 4894.16, 4890.97, 4927.93, 4924.97,
        4845.65, 4906.19, 4958.61, 4942.81, 4954.23, 4995.06, 4997.91, 5026.61, 5021.84, 4953.17,
        5000.62, 5029.73, 5005.57, 4975.51, 4981.8, 5087.03, 5088.8, 5069.53, 5078.18, 5069.76,
        5096.27, 5137.08, 5130.95, 5078.65, 5104.76, 5157.36, 5123.69, 5117.94, 5175.27, 5165.31,
        5150.48, 5117.09, 5149.42, 5178.51, 5224.62, 5241.53, 5234.18, 5218.19, 5203.58, 5248.49,
        5254.35, 5243.77, 5205.81, 5211.49, 5147.21, 5204.34, 5202.39, 5209.91, 5160.64, 5199.06,
        5123.41, 5061.82, 5051.41, 5022.21, 5011.12, 4967.23, 5010.6, 5070.55, 5071.63, 5048.42,
        5099.96, 5116.17, 5035.69, 5018.39, 5064.2, 5127.79, 5180.74, 5187.7, 5187.67, 5214.08,
        5222.68, 5221.42, 5246.68, 5308.15, 5297.1, 5303.27, 5308.13, 5321.41, 5307.01, 5267.84,
        5304.72, 5306.04, 5266.95, 5235.48, 5277.51, 5283.4, 5291.34, 5354.03, 5352.96, 5346.99,
        5360.79, 5375.32,
    ];

    let typical_price = vec![
        4327.81, 4364.56, 4367.42, 4409.21, 4421.83, 4385.35, 4370.68, 4371.99, 4352.07, 4339.65,
        4365.94, 4375.81, 4388.93, 4443.77, 4451.45, 4445.83, 4406.42, 4412.25, 4404.02, 4430.45,
        4474.58, 4505.59, 4510.91, 4520.18, 4543.96, 4567.21, 4542.39, 4542.38, 4553.11, 4566.83,
        4565.6, 4557.68, 4578.8, 4585.44, 4576.29, 4523.36, 4502.31, 4497.64, 4509.81, 4489.03,
        4477.16, 4484.71, 4461.42, 4477.83, 4449.64, 4419.28, 4385.45, 4362.28, 4389.21, 4396.3,
        4425.21, 4403.39, 4393.49, 4429.28, 4476.48, 4510.04, 4515.77, 4519.46, 4502.38, 4466.07,
        4446.47, 4459.8, 4482.04, 4468.61, 4466.78, 4498.59, 4465.17, 4454.0, 4436.8, 4421.54,
        4344.96, 4331.32, 4326.22, 4284.17, 4268.4, 4293.78, 4298.69, 4283.06, 4242.35, 4250.91,
        4250.41, 4284.05, 4320.39, 4361.11, 4366.98, 4353.63, 4338.95, 4366.44, 4368.1, 4327.55,
        4295.74, 4241.25, 4220.7, 4242.16, 4200.2, 4149.58, 4125.95, 4159.08, 4180.82, 4227.08,
        4301.92, 4355.4, 4361.91, 4373.35, 4377.91, 4361.56, 4395.54, 4409.04, 4487.78, 4506.45,
        4502.69, 4511.27, 4538.28, 4535.28, 4556.7, 4557.48, 4552.42, 4554.51, 4561.79, 4558.31,
        4582.91, 4562.96, 4565.81, 4562.19, 4580.58, 4595.89, 4613.18, 4631.91, 4686.67, 4717.49,
        4716.47, 4738.55, 4760.26, 4724.73, 4734.6, 4754.78, 4772.64, 4778.62, 4785.88, 4770.08,
        4739.94, 4711.27, 4701.0, 4700.28, 4742.63, 4750.77, 4776.82, 4772.77, 4785.07, 4765.15,
        4732.75, 4769.1, 4822.58, 4854.3, 4858.48, 4879.39, 4887.22, 4893.04, 4914.88, 4924.11,
        4865.85, 4888.89, 4947.3, 4939.36, 4948.96, 4988.0, 4995.13, 5019.0, 5029.02, 4948.26,
        4986.53, 5020.63, 5014.6, 4974.75, 4970.34, 5073.42, 5093.77, 5078.7, 5072.05, 5068.49,
        5087.72, 5123.86, 5135.93, 5083.34, 5108.32, 5150.4, 5143.48, 5111.25, 5156.54, 5165.44,
        5150.21, 5119.43, 5156.83, 5163.47, 5207.45, 5247.76, 5236.71, 5221.12, 5214.05, 5237.22,
        5255.01, 5245.64, 5199.4, 5211.54, 5183.29, 5194.58, 5206.44, 5198.5, 5159.26, 5183.2,
        5135.46, 5094.24, 5057.03, 5035.81, 5023.22, 4979.94, 5006.28, 5058.21, 5069.38, 5032.25,
        5095.91, 5109.44, 5060.61, 5042.65, 5049.49, 5122.71, 5168.05, 5188.96, 5181.83, 5203.26,
        5224.01, 5223.28, 5238.34, 5294.39, 5306.26, 5297.44, 5311.95, 5314.53, 5305.4, 5288.88,
        5298.25, 5300.95, 5270.64, 5239.26, 5249.84, 5273.28, 5282.59, 5335.28, 5350.22, 5351.13,
        5352.7, 5359.51, 5425.8,
    ];

    let length = close.len();

    let nominator = 5.0;
    let denominator = 4.0;

    let available_models = vec![
        rust_ti::ConstantModelType::SimpleMovingAverage,
        rust_ti::ConstantModelType::SmoothedMovingAverage,
        rust_ti::ConstantModelType::ExponentialMovingAverage,
        rust_ti::ConstantModelType::PersonalisedMovingAverage(&nominator, &denominator),
        rust_ti::ConstantModelType::SimpleMovingMedian,
        rust_ti::ConstantModelType::SimpleMovingMode,
    ];
    let available_deviations = vec![
        rust_ti::DeviationModel::StandardDeviation,
        rust_ti::DeviationModel::MeanAbsoluteDeviation,
        rust_ti::DeviationModel::MedianAbsoluteDeviation,
        rust_ti::DeviationModel::ModeAbsoluteDeviation,
        rust_ti::DeviationModel::UlcerIndex,
    ];

    let available_moving_averages = vec![
        rust_ti::MovingAverageType::Simple,
        rust_ti::MovingAverageType::Smoothed,
        rust_ti::MovingAverageType::Exponential,
        rust_ti::MovingAverageType::Personalised(&nominator, &denominator),
    ];

    let period: usize = 5;
    let previous_mcginley_dynamic = 0.0;
    let long_period: usize = 20;
    // basic_indicators.rs

    // Median

    let median = rust_ti::basic_indicators::bulk::median(&typical_price, &period);
    println!("Median vs Typical Price: {:?}", median);

    // Mode

    let mode = rust_ti::basic_indicators::bulk::mode(&typical_price, &period);
    println!("Mode vs Typical Price: {:?}", mode);

    // Natural logarithm

    let log = rust_ti::basic_indicators::bulk::log(&typical_price);
    println!("Natural log of Typical Price: {:?}", log);

    // Log difference

    let log_diff = rust_ti::basic_indicators::bulk::log_difference(&typical_price);
    println!("Log diff of Typical Price: {:?}", log_diff);

    // Variance

    let variance = rust_ti::basic_indicators::bulk::variance(&typical_price, &period);
    println!("Variance of Typical Price: {:?}", variance);

    // Standard Deviation

    let standard_dev = rust_ti::basic_indicators::bulk::standard_deviation(&typical_price, &period);
    println!("Standard Deviation of Typical Price: {:?}", standard_dev);

    // Mean Absolute Deviation

    let mean_ad = rust_ti::basic_indicators::bulk::absolute_deviation(
        &typical_price,
        &period,
        &rust_ti::CentralPoint::Mean,
    );
    println!("Mean Absolute Deviation of Typical Price: {:?}", mean_ad);

    // Median Absolute Deviation

    let median_ad = rust_ti::basic_indicators::bulk::absolute_deviation(
        &typical_price,
        &period,
        &rust_ti::CentralPoint::Median,
    );
    println!(
        "Median Absolute Deviation of Typical Price: {:?}",
        median_ad
    );

    // Mode Absolute Deviation

    let mode_ad = rust_ti::basic_indicators::bulk::absolute_deviation(
        &typical_price,
        &period,
        &rust_ti::CentralPoint::Mode,
    );
    println!("Mode Absolute Deviation of Typical Price: {:?}", mode_ad);

    // candle_indicators.rs

    let difference = 2.0;

    // MA Envelope

    for model in &available_models {
        let envelope = rust_ti::candle_indicators::bulk::moving_constant_envelopes(
            &typical_price,
            &model,
            &difference,
            &period,
        );
        println!("{:?} Envelope of Typical Price: {:?}", model, envelope);
    }

    // McGinley Dynamic Envelope

    let mcginley_envelope = rust_ti::candle_indicators::bulk::mcginley_dynamic_envelopes(
        &typical_price,
        &difference,
        &previous_mcginley_dynamic,
        &period,
    );
    println!(
        "McGinley Envelope of Typical Price: {:?}",
        mcginley_envelope
    );

    // MA Bands

    let deviation_multiplier = 2.0;
    for model in &available_models {
        for deviation in &available_deviations {
            let bands = rust_ti::candle_indicators::bulk::moving_constant_bands(
                &typical_price,
                &model,
                &deviation,
                &deviation_multiplier,
                &period,
            );
            println!(
                "{:?} Band with {:?} of Typical Price: {:?}",
                model, deviation, bands
            );
        }
    }

    // McGinley Dynamic Bands

    for deviation in &available_deviations {
        let md_bands = rust_ti::candle_indicators::bulk::mcginley_dynamic_bands(
            &typical_price,
            &deviation,
            &deviation_multiplier,
            &previous_mcginley_dynamic,
            &period,
        );
        println!(
            "McGinley Dynamic Band with {:?} of Typical Price: {:?}",
            deviation, md_bands
        );
    }

    // Ichimoku Cloud

    let ichimoku_cloud =
        rust_ti::candle_indicators::bulk::ichimoku_cloud(&high, &low, &close, &9, &26, &52);

    println!("Ichimoku cloud: {:?}", ichimoku_cloud);

    // Donchian Channels

    let donchian_channels =
        rust_ti::candle_indicators::bulk::donchian_channels(&high, &low, &period);

    println!("Donchian Channels: {:?}", donchian_channels);

    // Keltner Channel

    let multiplier = 2.0;
    for model in &available_models {
        for atr_model in &available_models {
            let keltner_channel = rust_ti::candle_indicators::bulk::keltner_channel(
                &high[1..],
                &low[1..],
                &close[..length - 1],
                &model,
                &atr_model,
                &multiplier,
                &period,
            );
            println!(
                "Keltner Channel {:?} {:?}: {:?}",
                model, atr_model, keltner_channel
            );
        }
    }

    // Supertrend

    for model in &available_models {
        let supertrend = rust_ti::candle_indicators::bulk::supertrend(
            &high,
            &low,
            &close,
            &model,
            &multiplier,
            &period,
        );
        println!("Supertrend {:?}: {:?}", model, supertrend);
    }

    // momentum_indicators.rs

    // RSI

    for model in &available_models {
        let rsi = rust_ti::momentum_indicators::bulk::relative_strength_index(
            &typical_price,
            &model,
            &period,
        );
        println!("{:?} RSI: {:?}", model, rsi);
    }

    // McGinley Dynamic RSI
    let md_rsi = rust_ti::momentum_indicators::bulk::mcginley_dynamic_rsi(
        &typical_price,
        &previous_mcginley_dynamic,
        &previous_mcginley_dynamic,
        &period,
    );
    println!("McGinlet Dynamic RSI: {:?}", md_rsi);

    // Stochastic Oscillator

    let stochastic_oscillator =
        rust_ti::momentum_indicators::bulk::stochastic_oscillator(&typical_price, &period);
    println!("Stochastic Oscillator: {:?}", stochastic_oscillator);

    // Slow stochastic and slowest stochastic
    // Because different MA models may want to be used on the SO at different times
    // there is a nested loop to apply all models of slow SO to all models for slowest

    for model in &available_models {
        let slow_so = rust_ti::momentum_indicators::bulk::slow_stochastic(
            &stochastic_oscillator,
            &model,
            &period,
        );
        println!("Slow Stochastic {:?}: {:?}", model, slow_so);

        for slowest_model in &available_models {
            let slowest_so = rust_ti::momentum_indicators::bulk::slowest_stochastic(
                &slow_so,
                &slowest_model,
                &period,
            );
            println!("Slowest Stochastic {:?}: {:?}", slowest_model, slowest_so);
        }
    }

    // McGinley Dynamic slow stochasitic

    let md_slow_so = rust_ti::moving_average::bulk::mcginley_dynamic(
        &stochastic_oscillator,
        &previous_mcginley_dynamic,
        &period,
    );
    println!("McGinley Dynamic Slow Stochasitc: {:?}", md_slow_so);

    // McGinley Dynamic slowest stochasitic

    let md_slowest_so = rust_ti::moving_average::bulk::mcginley_dynamic(
        &md_slow_so,
        &previous_mcginley_dynamic,
        &period,
    );
    println!("McGinley Dynamic Slowest Stochasitc: {:?}", md_slowest_so);

    // Money Flow Index
    /*
        let mfi_min = -80.0;
        let mfi_max = -20.0;

        let mfi = rust_ti::momentum_indicators::bulk::money_flow_index(
            &typical_price,
            &volume
        );
        println!("Money Flow Index");
    */
    // Rate of Change

    let roc = rust_ti::momentum_indicators::bulk::rate_of_change(&typical_price);
    println!("Rate of Change: {:?}", roc);
    /*
        // On Balance Volume

        let previous_obv = 0.0;
        let obv = rust_ti::momentum_indicators::bulk::on_balance_volume(
            &typical_price,
            &volume,
            &previous_obv
        );
        println!("On Balance Volume");
    */
    // Commodity Channel Index

    let constant_multiplier = 0.015;
    for model in &available_models {
        for deviation_model in &available_deviations {
            let cci = rust_ti::momentum_indicators::bulk::commodity_channel_index(
                &typical_price,
                &model,
                &deviation_model,
                &constant_multiplier,
                &period,
            );
            println!(
                "Commoditiy Channel Index {:?} {:?}: {:?}",
                model, deviation_model, cci
            );
        }
    }

    // McGinley Dynamic CCI

    for deviation_model in &available_deviations {
        let md_cci = rust_ti::momentum_indicators::bulk::mcginley_dynamic_commodity_channel_index(
            &typical_price,
            &previous_mcginley_dynamic,
            &deviation_model,
            &constant_multiplier,
            &period,
        );
        println!(
            "McGinley Dynamic Commoditiy Channel Index {:?}: {:?}",
            deviation_model, md_cci
        );
    }

    // MACD

    for short_model in &available_models {
        for long_model in &available_models {
            let macd = rust_ti::momentum_indicators::bulk::macd_line(
                &typical_price,
                &period,
                &short_model,
                &long_period,
                &long_model,
            );
            println!("MACD {:?} {:?}: {:?}", short_model, long_model, macd);

            for signal_model in &available_models {
                let signal =
                    rust_ti::momentum_indicators::bulk::signal_line(&macd, &signal_model, &period);
                println!("Signal line {:?}: {:?}", signal_model, signal);
            }
        }
    }

    // McGinley Dynamic MACD

    let md_macd = rust_ti::momentum_indicators::bulk::mcginley_dynamic_macd_line(
        &typical_price,
        &period,
        &previous_mcginley_dynamic,
        &long_period,
        &previous_mcginley_dynamic,
    );
    let trimmed_md_macd = md_macd.iter().map(|x| x.0).collect::<Vec<f64>>();
    let md_signal = rust_ti::moving_average::bulk::mcginley_dynamic(
        &trimmed_md_macd,
        &previous_mcginley_dynamic,
        &period,
    );
    println!("McGinley Dynamic MACD: {:?}", md_macd);
    println!("McGinley Dynamic Signal: {:?}", md_signal);

    // Chaikin Oscillator
    /*
        let previous_ad = 0.0;
        for short_model in &available_models {
            for long_model in &available_models {
                let co = rust_ti::momentum_indicators::bulk::chaikin_oscillator(
                    &high,
                    &low,
                    &close,
                    &volume,
                    &period,
                    &long_period,
                    &previous_ad,
                    &short_model,
                    &long_model
                );
                let trimmed_co = co.iter().map(|x| x.0).collect<Vec<f64>>();
                println!("Chaikin Oscillator {} {}", short_model, long_model);
            };
        };

        // McGinley Dynamic Chaikin Oscillator

        let md_co = rust_ti::momentum_indicators::bulk::mcginley_dynamic_chaikin_oscillator(
            &high,
            &low,
            &close,
            &volume,
            &period,
            &long_period,
            &previous_ad,
            &previous_mcginley_dynamic,
            &previous_mcginley_dynamic
        );
        println!("McGinley Dynamic Chaikin Oscillator");
    */

    // Percentage Price Oscillator

    for model in &available_models {
        let ppo = rust_ti::momentum_indicators::bulk::percentage_price_oscillator(
            &typical_price,
            &period,
            &long_period,
            &model,
        );
        println!("Percentage Price Oscillator {:?}: {:?}", model, ppo);
        for moving_average in &available_moving_averages {
            let signal =
                rust_ti::moving_average::bulk::moving_average(&ppo, &moving_average, &period);
            println!("{:?} Signal Line: {:?}", moving_average, signal);
        }
    }

    // Chande Momentum Oscillator
    let cmo =
        rust_ti::momentum_indicators::bulk::chande_momentum_oscillator(&typical_price, &period);
    println!("Chande Momentum Oscillator: {:?}", cmo);

    // moving_average.rs

    // Moving Averages

    for moving_average in &available_moving_averages {
        let ma =
            rust_ti::moving_average::bulk::moving_average(&typical_price, &moving_average, &period);
        println!("{:?} Moving Average: {:?}", moving_average, ma);
    }

    // McGinley Dynamic

    let mcginley_dynamic =
        rust_ti::moving_average::bulk::mcginley_dynamic(&typical_price, &0.0, &period);
    println!("McGinley Dynamic vs Typical Price:  {:?}", mcginley_dynamic);

    // other_indicators.rs

    // True Range

    let true_range = rust_ti::other_indicators::bulk::true_range(&close, &high, &low);
    println!("True Range: {:?}", true_range);

    // Average True Range

    for model in &available_models {
        let average_true_range = rust_ti::other_indicators::bulk::average_true_range(
            &close, &high, &low, &model, &period,
        );
        println!("{:?} Average True Range: {:?}", model, average_true_range);
    }

    // Internal Bar Strength

    let ibs = rust_ti::other_indicators::bulk::internal_bar_strength(&high, &low, &close);
    println!("Internal Bar Strength: {:?}", ibs);

    // Positivity Index

    for model in &available_models {
        let pi = rust_ti::other_indicators::bulk::positivity_indicator(
            &open[1..],
            &close[..length - 1],
            &period,
            &model,
        );
        println!("{:?} Positivity Index: {:?}", model, pi);
    }

    // strength_indicators.rs

    // Accumulation Distribution
    /*
        let ad = rust_ti::strength_indicators::bulk::accumulation_distribution(
            &high,
            &low,
            &close,
            &volume,
            &0.0
        );

        // Positive Volume Index

        let pvi = rust_ti::strength_indicators::bulk::positive_volume_index(
            &close,
            &volume,
            &0.0
        );

        // Negative Volume Index

        let nvi = rust_ti::strength_indicators::bulk::negative_volume_index(
            &close,
            &volume,
            &0.0
        );
    */

    // Relative Vigor Index

    for model in &available_models {
        let rvi = rust_ti::strength_indicators::bulk::relative_vigor_index(
            &open, &high, &low, &close, &model, &period,
        );
        println!("{:?} Relative Vigor Index: {:?}", model, rvi);
        for signal_model in &available_moving_averages {
            let signal =
                rust_ti::moving_average::bulk::moving_average(&rvi, &signal_model, &period);
            println!("{:?} Signal Line: {:?}", signal_model, signal);
        }
    }

    // trend_indicators.rs

    // Aroon Indicator

    let aroon_indicator = rust_ti::trend_indicators::bulk::aroon_indicator(&high, &low, &period);
    println!("Aroon Indicator: {:?}", aroon_indicator);

    // Parabolic Time Price System

    let ptps = rust_ti::trend_indicators::bulk::parabolic_time_price_system(
        &high,
        &low,
        &0.02,
        &0.2,
        &0.02,
        &rust_ti::Position::Long,
        &0.0,
    );
    println!("Parabolic Time Price System: {:?}", ptps);

    // Directional Movement System

    for model in &available_models {
        let dms = rust_ti::trend_indicators::bulk::directional_movement_system(
            &high, &low, &close, &period, &model,
        );
        println!("{:?} Directional Movement System: {:?}", model, dms);
    }

    // Volume Price Trend
    /*
        let vpt = rust_ti::trend_indicators::bulk::volume_price_trend(
            &typical_price,
            &volume,
            &0.0
        );
        println!("Volume Price Trend: {:?}", vpt);
    */

    // True Strength Index

    for first_model in &available_models {
        for second_model in &available_models {
            let tsi = rust_ti::trend_indicators::bulk::true_strength_index(
                &typical_price,
                &first_model,
                &period,
                &second_model,
                &period,
            );
            println!(
                "{:?} {:?} True Strength Index: {:?}",
                first_model, second_model, tsi
            );
            for signal_model in &available_moving_averages {
                let signal =
                    rust_ti::moving_average::bulk::moving_average(&tsi, &signal_model, &period);
                println!("{:?} Signal Line: {:?}", signal_model, signal);
            }
        }
    }

    // volatility_indicators.rs

    // Ulcer Index

    let ulcer_index = rust_ti::volatility_indicators::bulk::ulcer_index(&typical_price, &period);
    println!("Ulcer Index: {:?}", ulcer_index);

    // Volatility System

    let constant_multiplier = 3.0;
    for model in &available_models {
        let vs = rust_ti::volatility_indicators::bulk::volatility_system(
            &high,
            &low,
            &close,
            &period,
            &constant_multiplier,
            &model,
        );
        println!("{:?} Volatility System: {:?}", model, vs);
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
