#[doc = "Register `CMD_W0` reader"]
pub type R = crate::R<CMD_W0_SPEC>;
#[doc = "Register `CMD_W0` writer"]
pub type W = crate::W<CMD_W0_SPEC>;
#[doc = "Field `SPIADR` reader - SPI flash address"]
pub type SPIADR_R = crate::FieldReader<u32>;
#[doc = "Field `SPIADR` writer - SPI flash address"]
pub type SPIADR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SPI flash address"]
    #[inline(always)]
    pub fn spiadr(&self) -> SPIADR_R {
        SPIADR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMD_W0")
            .field("spiadr", &self.spiadr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - SPI flash address"]
    #[inline(always)]
    pub fn spiadr(&mut self) -> SPIADR_W<'_, CMD_W0_SPEC> {
        SPIADR_W::new(self, 0)
    }
}
#[doc = "Command word 0\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd_w0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd_w0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMD_W0_SPEC;
impl crate::RegisterSpec for CMD_W0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd_w0::R`](R) reader structure"]
impl crate::Readable for CMD_W0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmd_w0::W`](W) writer structure"]
impl crate::Writable for CMD_W0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMD_W0 to value 0"]
impl crate::Resettable for CMD_W0_SPEC {}
